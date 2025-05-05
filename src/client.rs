use crate::io::{Args, GremlinIO, Request};
use crate::prelude::{ConnectionOptions, GValue};
use crate::structure::GKey;
use crate::{GremlinError, GremlinResult};
use bb8::{Pool, PooledConnection};
use futures::Stream;
use serde::Serialize;
use std::collections::HashMap;

pub struct SessionedClient<'c, V: GremlinIO> {
    connection: PooledConnection<'c, ConnectionOptions<V>>,
    session: Option<String>,
    alias: Option<String>,
}

// impl<'c, V: GremlinIO> SessionedClient<'c, SD> {
//     pub async fn close_session(mut self) -> GremlinResult<GResultSet<V>> {
//         if let Some(session_name) = self.session.take() {
//             let mut args = HashMap::new();
//             args.insert(String::from("session"), GValue::from(session_name.clone()));
//             let args = SD::serialize(&GValue::from(args))?;
//
//             let processor = "session".to_string();
//
//             let message = SD::message(String::from("close"), processor, args, None);
//
//             GremlinClient::send_message_new(self.connection, message).await
//         } else {
//             Err(GremlinError::Generic("No session to close".to_string()))
//         }
//     }
// }

#[derive(Clone)]
pub struct GremlinClient<V: GremlinIO> {
    pool: bb8::Pool<ConnectionOptions<V>>,
    session: Option<String>,
    alias: Option<String>,
    // pub(crate) options: ConnectionOptions<V>,
}

const G: &'static str = "g";
const GREMLIN: &'static str = "gremlin";
const LANGUAGE: &'static str = "language";
const GREMLIN_GROOVY: &'static str = "gremlin-grovy";
const ALIASES: &'static str = "aliases";
const BINDINGS: &'static str = "bindings";
const SESSION: &'static str = "session";
const EVAL: &'static str = "eval";

impl<V> GremlinClient<V>
where
    V: GremlinIO,
{
    pub async fn connect(options: ConnectionOptions<V>) -> GremlinResult<GremlinClient<V>> {
        let pool = Pool::builder()
            .min_idle(3)
            .max_size(options.pool_size)
            .idle_timeout(options.idle_timeout)
            .connection_timeout(options.connection_timeout)
            .build(options)
            .await?;

        Ok(GremlinClient {
            pool,
            session: None,
            alias: None,
        })
    }

    pub async fn create_session(&mut self, name: String) -> GremlinResult<SessionedClient<V>> {
        let connection = self.pool.get().await?;

        Ok(SessionedClient {
            connection,
            session: Some(name),
            alias: None,
        })
    }

    /// Return a cloned client with the provided alias
    pub fn alias<T>(&mut self, alias: T) -> GremlinClient<V>
    where
        T: Into<String>,
    {
        let mut cloned = self.clone();
        cloned.alias = Some(alias.into());
        cloned
    }

    pub async fn execute_raw<T>(
        &self,
        script: T,
        params: &[(&str, &dyn Into<GValue>)],
    ) -> GremlinResult<impl Stream<Item = GremlinResult<GValue>>>
    where
        T: Into<String>,
    {
        let args2 = Args::new()
            .arg(GREMLIN, script)
            .arg(LANGUAGE, GREMLIN_GROOVY)
            .arg(
                ALIASES,
                 self.alias
                     .iter()
                     .map(|string| (GKey::String(G.into()), GValue::String(string.clone())))
                     .collect::<HashMap<_, _>>(),
            )
            .arg(
                BINDINGS,
                params
                    .iter()
                    .map(|(k, v)| ((*k).into(), v.to_gvalue()))
                    .collect::<HashMap<GKey, _>>(),
            )
        ;
        let args = {
            let mut tmp = HashMap::new();

            tmp.insert(GREMLIN, GValue::String(script.into()));
            tmp.insert(LANGUAGE, GValue::String(GREMLIN_GROOVY.into()));
            tmp.insert(
                ALIASES,
                GValue::from(
                    self.alias
                        .iter()
                        .map(|string| (GKey::String(G.into()), GValue::String(string.clone())))
                        .collect::<HashMap<_, _>>(),
                ),
            );
            tmp.insert(
                BINDINGS,
                GValue::from(
                    params
                        .iter()
                        .map(|(k, v)| ((*k).into(), v.to_gvalue()))
                        .collect::<HashMap<GKey, _>>(),
                ),
            );
            if let Some(session_name) = &self.session {
                tmp.insert(SESSION, GValue::from(session_name.clone()));
            }

            V::serialize(&GValue::from(tmp))
        }?;
        let processor = if self.session.is_some() {
            SESSION
        } else {
            "traversal" // TODO ?
        };

        let request = Request::builder()
            .op(EVAL)
            .proc(processor)
            .args(args2)
            .build()
            .unwrap();

        let conn = self.pool.get().await?;
        let stream = conn.send::<_, V>(request).await?;

        Ok(stream)
    }

    // pub async fn submit_traversal(&self, bytecode: &Bytecode) -> GremlinResult<GResultSet<V>> {
    //     tracing::trace!("{:?}", bytecode);
    //
    //     let mut args = HashMap::new();
    //
    //     args.insert(String::from("gremlin"), GValue::Bytecode(bytecode.clone()));
    //
    //     let aliases = self
    //         .alias
    //         .clone()
    //         .or_else(|| Some(String::from("g")))
    //         .map(|s| {
    //             let mut map = HashMap::new();
    //             map.insert(String::from("g"), GValue::String(s));
    //             map
    //         })
    //         .unwrap_or_else(HashMap::new);
    //
    //     args.insert(String::from("aliases"), GValue::from(aliases));
    //
    //     let args = SD::serialize(&GValue::from(args))?;
    //
    //     let message = SD::message(
    //         String::from("bytecode"),
    //         String::from("traversal"),
    //         args,
    //         None,
    //     );
    //
    //     let conn = self.pool.get().await?;
    //
    //     // self.send_message_new(conn, message).await
    // }
}

// fn build_message<T: Serialize>(msg: Message<T>) -> GremlinResult<String> {
//     serde_json::to_string(&msg).map_err(GremlinError::from)
// }
