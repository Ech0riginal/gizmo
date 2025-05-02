use crate::io::GraphSON;
use crate::pool::GremlinConnectionManager;
use crate::prelude::{
    ConnectionOptions, GResultSet, GValue, GremlinError, GremlinResult, Message, ToGValue,
    traversal::Bytecode,
};
use base64::prelude::{BASE64_STANDARD, Engine};
use futures::future::{BoxFuture, FutureExt};
use mobc::{Connection, Pool};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};

pub type SessionedClient<SD> = GremlinClient<SD>;

impl<SD: GraphSON> SessionedClient<SD> {
    pub async fn close_session(&mut self) -> GremlinResult<GResultSet<SD>> {
        if let Some(session_name) = self.session.take() {
            let mut args = HashMap::new();
            args.insert(String::from("session"), GValue::from(session_name.clone()));
            let args = SD::serialize(&GValue::from(args))?;

            let processor = "session".to_string();

            let message = SD::message(String::from("close"), processor, args, None);

            let conn = self.pool.get().await?;

            self.send_message_new(conn, message).await
        } else {
            Err(GremlinError::Generic("No session to close".to_string()))
        }
    }
}

#[derive(Clone)]
pub struct GremlinClient<SD: GraphSON> {
    pool: Pool<GremlinConnectionManager<SD>>,
    session: Option<String>,
    alias: Option<String>,
    pub(crate) options: ConnectionOptions<SD>,
}

impl<SD: GraphSON> GremlinClient<SD> {
    pub async fn connect<T>(options: T) -> GremlinResult<GremlinClient<SD>>
    where
        T: Into<ConnectionOptions<SD>>,
    {
        let opts = options.into();
        let pool_size = opts.pool_size;
        let manager = GremlinConnectionManager::new(opts.clone());

        let pool = Pool::builder()
            .get_timeout(opts.pool_get_connection_timeout)
            .max_open(pool_size as u64)
            //Makes max idle connections equal to max open, matching the behavior of the sync pool r2d2
            .max_idle(0)
            .health_check_interval(opts.pool_healthcheck_interval)
            .build(manager);

        Ok(GremlinClient {
            pool,
            session: None,
            alias: None,
            options: opts,
        })
    }

    pub async fn create_session(&mut self, name: String) -> GremlinResult<SessionedClient<SD>> {
        let manager = GremlinConnectionManager::new(self.options.clone());
        Ok(SessionedClient {
            pool: Pool::builder().max_open(1).build(manager),
            session: Some(name),
            alias: None,
            options: self.options.clone(),
        })
    }

    /// Return a cloned client with the provided alias
    pub fn alias<T>(&mut self, alias: T) -> GremlinClient<SD>
    where
        T: Into<String>,
    {
        let mut cloned = self.clone();
        cloned.alias = Some(alias.into());
        cloned
    }

    pub async fn execute<T>(
        &self,
        script: T,
        params: &[(&str, &dyn ToGValue)],
    ) -> GremlinResult<GResultSet<SD>>
    where
        T: Into<String>,
    {
        let mut args = HashMap::new();

        args.insert(String::from("gremlin"), GValue::String(script.into()));
        args.insert(
            String::from("language"),
            GValue::String(String::from("gremlin-groovy")),
        );

        let aliases = self
            .alias
            .clone()
            .map(|s| {
                let mut map = HashMap::new();
                map.insert(String::from("g"), GValue::String(s));
                map
            })
            .unwrap_or_else(HashMap::new);

        args.insert(String::from("aliases"), GValue::from(aliases));

        let bindings: HashMap<String, GValue> = params
            .iter()
            .map(|(k, v)| (String::from(*k), v.to_gvalue()))
            .collect();

        args.insert(String::from("bindings"), GValue::from(bindings));

        if let Some(session_name) = &self.session {
            args.insert(String::from("session"), GValue::from(session_name.clone()));
        }

        let args = SD::serialize(&GValue::from(args))?;

        let processor = if self.session.is_some() {
            "session".to_string()
        } else {
            String::default()
        };

        let message = SD::message(String::from("eval"), processor, args, None);

        let conn = self.pool.get().await?;

        self.send_message_new(conn, message).await
    }

    pub(crate) fn send_message_new<'a, T: Serialize>(
        &'a self,
        mut conn: Connection<GremlinConnectionManager<SD>>,
        msg: Message<T>,
    ) -> BoxFuture<'a, GremlinResult<GResultSet<SD>>> {
        let id = msg.id().clone();
        let message = self.build_message(msg).unwrap();

        async move {
            let span = tracing::span!(tracing::Level::DEBUG, "gremlin");
            let _enter = span.enter();

            tracing::trace!(parent: &span, request=&id.to_string());

            let content_type = SD::content_type();
            let payload = String::new() + content_type + &message;
            let mut binary = payload.into_bytes();
            binary.insert(0, content_type.len() as u8);

            let (response, receiver) = conn.send(id.clone(), binary).await?;

            // tracing::debug!(parent: &span, request=format!("{}", &id), code=response.status.code);

            let (response, results) = match response.status.code {
                200 | 206 => {
                    let results: VecDeque<GValue> = SD::deserialize(&response.result.data)?.into();

                    Ok((response, results))
                }
                204 => Ok((response, VecDeque::new())),
                407 => match &self.options.credentials {
                    Some(c) => {
                        let mut args = HashMap::new();

                        args.insert(
                            String::from("sasl"),
                            GValue::String(
                                BASE64_STANDARD
                                    .encode(&format!("\0{}\0{}", c.username, c.password)),
                            ),
                        );

                        let args = SD::serialize(&GValue::from(args))?;
                        let message = SD::message(
                            String::from("authentication"),
                            String::from("traversal"),
                            args,
                            Some(response.request_id),
                        );

                        return self.send_message_new(conn, message).await;
                    }
                    None => Err(GremlinError::Request((
                        response.status.code,
                        response.status.message,
                    ))),
                },
                _ => Err(GremlinError::Request((
                    response.status.code,
                    response.status.message,
                ))),
            }?;

            Ok(GResultSet::new(self.clone(), results, response, receiver))
        }
        .boxed()
    }

    pub async fn submit_traversal(&self, bytecode: &Bytecode) -> GremlinResult<GResultSet<SD>> {
        tracing::trace!("{:?}", bytecode);

        let mut args = HashMap::new();

        args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));

        let aliases = self
            .alias
            .clone()
            .or_else(|| Some(String::from("g")))
            .map(|s| {
                let mut map = HashMap::new();
                map.insert(String::from("g"), GValue::String(s));
                map
            })
            .unwrap_or_else(HashMap::new);

        args.insert(String::from("aliases"), GValue::from(aliases));

        let args = SD::serialize(&GValue::from(args))?;

        let message = SD::message(
            String::from("bytecode"),
            String::from("traversal"),
            args,
            None,
        );

        let conn = self.pool.get().await?;

        self.send_message_new(conn, message).await
    }

    fn build_message<T: Serialize>(&self, msg: Message<T>) -> GremlinResult<String> {
        serde_json::to_string(&msg).map_err(GremlinError::from)
    }
}
