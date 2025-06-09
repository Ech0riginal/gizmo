use crate::Error;
use crate::io::GremlinIO;
use crate::options::ConnectionOptions;
use bb8::ManageConnection;
use std::sync::Arc;
use tokio_tungstenite::{Connector, connect_async_with_config};
use tungstenite::{
    client::{IntoClientRequest, uri_mode},
    stream::{Mode, NoDelay},
};

impl<V> ManageConnection for ConnectionOptions<V>
where
    V: GremlinIO,
{
    type Connection = super::Connection<V>;
    type Error = Error;

    fn connect(&self) -> impl Future<Output = Result<Self::Connection, Self::Error>> + Send {
        async move {
            let websocket_url = self.websocket_url();
            let request = websocket_url
                .clone()
                .into_client_request()
                .map_err(|e| Error::Generic(e.to_string()))?;

            let connector = if let Some(opts) = &self.tls_options {
                let config = opts.clone().config()?;
                let config = Arc::new(config);
                Connector::Rustls(config)
            } else {
                Connector::Plain
            };
            let url = request.uri();
            let mode = uri_mode(url).map_err(|e| Error::Generic(e.to_string()))?;
            let host = request
                .uri()
                .host()
                .ok_or_else(|| Error::Generic("No Hostname".into()))?;
            let port = url.port_u16().unwrap_or(match mode {
                Mode::Plain => 80,
                Mode::Tls => 443,
            });
            let mut stream = std::net::TcpStream::connect((host, port))
                .map_err(|e| Error::Generic(format!("Unable to connect {e:?}")))?;
            NoDelay::set_nodelay(&mut stream, true)
                .map_err(|e| Error::Generic(e.to_string()))?;

            let websocket_config = self.websocket_options.clone().map(Into::into);

            let (client, _) = match &connector {
                Connector::Plain => connect_async_with_config(url, websocket_config, false).await,
                Connector::Rustls(_) => {
                    tokio_tungstenite::connect_async_tls_with_config(
                        url,
                        websocket_config,
                        false,
                        Some(connector),
                    )
                    .await
                }
                _ => panic!("NativeTls isn't supported :D"),
            }?;

            Ok(super::Connection::new(client, self.clone()))
        }
    }

    fn is_valid(
        &self,
        conn: &mut Self::Connection,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            if !conn.valid().await {
                Err(Error::Generic("Connection is disconnected".into()))
            } else {
                Ok(())
            }
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.healthcheck()
    }
}
