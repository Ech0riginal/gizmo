use crate::connection::Connection;
use crate::prelude::{
    ConnectionOptions, GResultSet, GValue, Message, ToGValue,
    traversal::Bytecode,
};
use crate::structure::GKey;
use base64::prelude::{BASE64_STANDARD, Engine};
use bb8::{Pool, PooledConnection, RunError};
use futures::{Stream, StreamExt};
use futures::future::{BoxFuture, FutureExt};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use tokio::pin;
use crate::{Gremlin, GremlinError, GremlinResult};

pub struct SessionedClient<'c, SD: Gremlin> {
    connection: PooledConnection<'c, ConnectionOptions<SD>>,
    session: Option<String>,
    alias: Option<String>,
}

// impl<'c, SD: Gremlin> SessionedClient<'c, SD> {
//     pub async fn close_session(mut self) -> GremlinResult<GResultSet<SD>> {
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
pub struct GremlinClient<SD: Gremlin> {
    pool: bb8::Pool<ConnectionOptions<SD>>,
    session: Option<String>,
    alias: Option<String>,
    // pub(crate) options: ConnectionOptions<SD>,
}

const G: &'static str = "g";
const GREMLIN: &'static str = "gremlin";
const LANGUAGE: &'static str = "language";
const GREMLIN_GROOVY: &'static str = "gremlin-grovy";
const ALIASES: &'static str = "aliases";
const BINDINGS: &'static str = "bindings";
const SESSION: &'static str = "session";
const EVAL: &'static str = "eval";

impl<SD: Gremlin> GremlinClient<SD> {
    pub async fn connect(options: ConnectionOptions<SD>) -> GremlinResult<GremlinClient<SD>> {
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

    pub async fn create_session(&mut self, name: String) -> GremlinResult<SessionedClient<SD>> {
        let connection = self
            .pool
            .get()
            .await?;

        Ok(SessionedClient {
            connection,
            session: Some(name),
            alias: None,
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
    ) -> GremlinResult<impl Stream<Item = GremlinResult<GValue>>>
    where
        T: Into<String>,
    {
        todo!();
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

            todo!()
            // SD::serialize(&GValue::from(tmp))
        }?;
        let processor = if self.session.is_some() {
            SESSION.into()
        } else {
            String::default()
        };
        
        todo!()
        // let message = SD::message(EVAL.into(), processor, args, None);
        // let conn = self.pool.get().await?;
        // let stream = conn.send::<_, SD>(message).await?;
        // Ok(stream)
    }

    // pub async fn submit_traversal(&self, bytecode: &Bytecode) -> GremlinResult<GResultSet<SD>> {
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

fn build_message<T: Serialize>(msg: Message<T>) -> GremlinResult<String> {
    serde_json::to_string(&msg).map_err(GremlinError::from)
}
