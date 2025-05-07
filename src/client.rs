use crate::io::{Args, GremlinIO, Request};
use crate::options::ConnectionOptions;
use crate::structure::*;
use crate::{GremlinError, GremlinResult};
use bb8::{Pool, PooledConnection};
use std::collections::HashMap;
use crate::network::{GremlinStream};

pub struct SessionedClient<'c, V: GremlinIO> {
    connection: PooledConnection<'c, ConnectionOptions<V>>,
    session: Option<String>,
    alias: Option<String>,
}

impl<'a, V: GremlinIO> SessionedClient<'a, V> {
    pub async fn close(mut self) -> GremlinResult<impl GremlinStream + 'a> {
        if let Some(session_name) = self.session.take() {
            let request = Request::builder()
                .op(CLOSE)
                .proc(SESSION)
                .args(Args::new().arg(SESSION, &session_name))
                .build().unwrap();
            self.connection.send(request).await
        } else {
            Err(GremlinError::Generic("No session to close".to_string()))
        }
    }
}

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
const CLOSE: &'static str = "close";
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

    pub async fn execute_raw<'a, S>(
        &'a self,
        script: S,
        params: &'a [(&'a str, impl Into<GValue> + Clone)],
    ) -> GremlinResult<impl GremlinStream + 'a>
    where
        S: AsRef<str>,
    {
        let args = Args::new()
            .arg(GREMLIN, script.as_ref())
            .arg(LANGUAGE, GREMLIN_GROOVY)
            .arg(
                ALIASES,
                [()].iter().map(|_| (
                    G,
                    self.alias
                        .clone()
                        .map(|str| GValue::String(str.into()))
                        .unwrap_or(GValue::String(G.into()))
                )).collect::<HashMap<_, GValue>>()
            )
            .arg(
                BINDINGS,
                params
                    .into_iter()
                    .map(|(k, v)| (*k, v.clone().into()))
                    .collect::<HashMap<_, GValue>>(),
            )
            .arg(SESSION, self.session.clone());

        let processor = if self.session.is_some() {
            SESSION
        } else {
            "traversal" // TODO ?
        };

        let request = Request::builder()
            .op(EVAL)
            .proc(processor)
            .args(args)
            .build()
            .unwrap();
        let conn = self.pool.get().await?;
        let stream = conn.send(request).await?;
        
        Ok(stream)
    }

    pub async fn execute(
        &self,
        bytecode: Bytecode,
    ) -> GremlinResult<impl GremlinStream + '_> {
        let bytecode = GValue::Bytecode(bytecode);
        let request = Request::builder()
            .op("bytecode")
            .proc("traversal")
            .args(Args::new()
                .arg(GREMLIN, bytecode)
                .arg(
                    ALIASES,
                    [()].iter().map(|_| (
                        G,
                            self.alias
                            .clone()
                            .map(|str| GValue::String(str.into()))
                            .unwrap_or(GValue::String(G.into()))
                    )).collect::<HashMap<_, GValue>>()
              ))
            .build()
            .unwrap();
        let conn = self.pool.get().await?;
        let stream = conn.send(request).await?;

        Ok(stream)
    }
}
