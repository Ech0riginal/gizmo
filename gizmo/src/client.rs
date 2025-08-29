// use crate::io::{Args, GremlinIO, Request};
use std::collections::HashMap;

use bb8::{Pool, PooledConnection};
use gizmio::types::{Bytecode, GValue};
use gizmio::{Args, Deserializer, Dialect, Format, Request, Response, Serializer};

use crate::network::Socketed;
use crate::options::ConnectionOptions;
use crate::{Error, GremlinResult};

pub struct SessionedClient<'c, D: Dialect, F: Supports<D>>
where
    <F as Format>::Serial: Send + Sync,
{
    connection: PooledConnection<'c, ConnectionOptions<D, F>>,
    session: Option<String>,
    alias: Option<String>,
}

impl<'a, D, F> SessionedClient<'a, D, F>
where
    D: Dialect,
    F: Supports<D>,
    F::Serial: Send + Sync,
{
    pub async fn close(mut self) -> GremlinResult<Socketed> {
        if let Some(session_name) = self.session.take() {
            let request = Request::builder()
                .op(CLOSE)
                .proc(SESSION)
                .args(Args::new().arg(SESSION, &session_name))
                .build()
                .unwrap();
            self.connection.send(request).await
        } else {
            Err(Error::Generic("No session to close".to_string()))
        }
    }
}

pub trait Supports<D: Dialect>:
    Format + Serializer<Request, Self::Serial, D> + Deserializer<Response, Self::Serial, D>
{
}
impl<D, T> Supports<D> for T
where
    D: Dialect,
    T: Format,
    T: Serializer<Request, Self::Serial, D> + Deserializer<Response, Self::Serial, D>,
    T::Serial: Send + Sync,
{
}

#[derive(Clone)]
pub struct GremlinClient<D: Dialect, F: Supports<D>>
where
    <F as Format>::Serial: Send + Sync,
{
    pool: bb8::Pool<ConnectionOptions<D, F>>,
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

impl<D, F> GremlinClient<D, F>
where
    D: Dialect,
    F: Supports<D>,
    F::Serial: Send + Sync,
{
    pub async fn connect(options: ConnectionOptions<D, F>) -> GremlinResult<GremlinClient<D, F>> {
        let pool = Pool::builder()
            .min_idle(1)
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

    pub async fn create_session(
        &mut self,
        name: String,
    ) -> GremlinResult<SessionedClient<'_, D, F>> {
        let connection = self.pool.get().await?;

        Ok(SessionedClient {
            connection,
            session: Some(name),
            alias: None,
        })
    }

    /// Return a cloned client with the provided alias
    pub fn alias<T>(&mut self, alias: T) -> GremlinClient<D, F>
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
    ) -> GremlinResult<Socketed>
    where
        S: AsRef<str>,
    {
        let args = Args::new()
            .arg(GREMLIN, script.as_ref())
            .arg(LANGUAGE, GREMLIN_GROOVY)
            .arg(
                ALIASES,
                [()].iter()
                    .map(|_| {
                        (
                            G,
                            self.alias
                                .clone()
                                .map(|str| GValue::String(str.into()))
                                .unwrap_or(GValue::String(G.into())),
                        )
                    })
                    .collect::<HashMap<_, GValue>>(),
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
        let mut conn = self.pool.get().await?;
        let socket = conn.send(request).await?;

        Ok(socket)
    }

    pub async fn execute<'a>(&self, bytecode: Bytecode) -> GremlinResult<Socketed> {
        let bytecode = GValue::Bytecode(bytecode);
        let request = Request::builder()
            .op("bytecode")
            .proc("traversal")
            .args(
                Args::new().arg(GREMLIN, bytecode).arg(
                    ALIASES,
                    [()].iter()
                        .map(|_| {
                            (
                                G,
                                self.alias
                                    .clone()
                                    .map(|str| GValue::String(str.into()))
                                    .unwrap_or(GValue::String(G.into())),
                            )
                        })
                        .collect::<HashMap<_, GValue>>(),
                ),
            )
            .build()
            .unwrap();
        tracing::trace!("Acquiring connection from pool");
        let mut conn = self.pool.get().await?;
        tracing::trace!("Acquired connection from pool; Sending request");
        let socket = conn.send(request).await?;
        Ok(socket)
    }
}
