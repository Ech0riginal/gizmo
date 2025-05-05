use crate::{Gremlin, GremlinError};
use mobc::Manager;

use crate::connection::Conn;
use crate::options::ConnectionOptions;
use crate::prelude::{GValue};
use base64::prelude::{BASE64_STANDARD, Engine};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct GremlinConnectionManager<SD: Gremlin> {
    options: ConnectionOptions<SD>,
}

impl<SD: Gremlin> GremlinConnectionManager<SD> {
    pub(crate) fn new(options: ConnectionOptions<SD>) -> GremlinConnectionManager<SD> {
        GremlinConnectionManager { options }
    }
}

#[async_trait::async_trait]
impl<SD: Gremlin> Manager for GremlinConnectionManager<SD> {
    type Connection = Conn;
    type Error = GremlinError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::connect(self.options.clone()).await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        let mut args = HashMap::new();

        args.insert(
            String::from("gremlin"),
            GValue::String("g.inject(0)".into()),
        );
        args.insert(
            String::from("language"),
            GValue::String(String::from("gremlin-groovy")),
        );
        let args = SD::serialize(&GValue::from(args))?;

        let message = SD::message(String::from("eval"), String::default(), args, None);

        let id = message.id().clone();
        let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

        let content_type = SD::mime();

        let payload = String::from("") + content_type + &msg;
        let mut binary = payload.into_bytes();
        binary.insert(0, content_type.len() as u8);

        let (response, _receiver) = conn.send(id, binary).await?;

        match response.status.code {
            200 | 206 => Ok(conn),
            204 => Ok(conn),
            407 => match &self.options.credentials {
                Some(c) => {
                    let mut args = HashMap::new();

                    args.insert(
                        String::from("sasl"),
                        GValue::String(
                            BASE64_STANDARD.encode(&format!("\0{}\0{}", c.username, c.password)),
                        ),
                    );

                    let args = SD::serialize(&GValue::from(args))?;
                    let message = SD::message(
                        String::from("authentication"),
                        String::from("traversal"),
                        args,
                        Some(response.request_id),
                    );

                    let id = message.id().clone();
                    let msg = serde_json::to_string(&message).map_err(GremlinError::from)?;

                    let content_type = SD::mime();
                    let payload = String::from("") + content_type + &msg;

                    let mut binary = payload.into_bytes();
                    binary.insert(0, content_type.len() as u8);

                    let (response, _receiver) = conn.send(id, binary).await?;

                    match response.status.code {
                        200 | 206 => Ok(conn),
                        204 => Ok(conn),
                        401 => Ok(conn),
                        // 401 is actually a username/password incorrect error, but if not
                        // not returned as okay, the pool loops infinitely trying
                        // to authenticate.
                        _ => Err(GremlinError::Request((
                            response.status.code,
                            response.status.message,
                        ))),
                    }
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
        }
    }

    fn validate(&self, conn: &mut Self::Connection) -> bool {
        conn.is_valid()
    }
}

#[cfg(test)]
mod tests {

    use super::GremlinConnectionManager;
    use crate::prelude::ConnectionOptions;

    use mobc::Pool;
    use std::time::Duration;

    use tokio::task;

    #[tokio::test]
    #[allow(unused_must_use)]
    async fn it_should_create_a_connection_pool() {
        let manager = GremlinConnectionManager::<()>::new(ConnectionOptions::default());

        let pool = Pool::builder().max_open(16).build(manager);

        let conn = pool.get().await.expect("Failed to get the connection");

        pool.state().await;

        assert_eq!(1, pool.state().await.connections);

        assert_eq!(0, pool.state().await.idle);

        drop(conn);

        task::spawn_blocking(move || {
            std::thread::sleep(Duration::from_millis(200));
        })
        .await;

        assert_eq!(1, pool.state().await.idle);
    }
}
