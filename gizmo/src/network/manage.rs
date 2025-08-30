use std::sync::Arc;

use bb8::ManageConnection;
use gizmio::{Deserializer, Dialect, Format, Request, Response, Serializer};
use tokio_tungstenite::{Connector, connect_async_with_config};
use tungstenite::{
    client::{IntoClientRequest, uri_mode},
    stream::{Mode, NoDelay},
};

use crate::Error;
use crate::options::ConnectionOptions;

impl<D, F> ManageConnection for ConnectionOptions<D, F>
where
    F: Format,
    F: Serializer<Request, F::Serial, D>,
    F: Deserializer<Response, F::Serial, D>,
    F::Serial: Send + Sync,
    D: Dialect,
{
    type Connection = super::Socketeer<D, F>;
    type Error = Error;

    fn connect(&self) -> impl Future<Output = Result<Self::Connection, Self::Error>> + Send {
        async move {
            let request = self
                .websocket_url()
                .into_client_request()
                .map_err(|e| Error::Generic(e.to_string()))?;

            let connector = if let Some(opts) = &self.tls_options {
                let config: rustls::ClientConfig = opts.clone().config()?;
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
            NoDelay::set_nodelay(&mut stream, true).map_err(|e| Error::Generic(e.to_string()))?;

            let websocket_config = self.websocket_options.clone().map(Into::into);

            let (stream, _) = match &connector {
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

            tracing::trace!("Connected to ws://{}:{}", host, port);

            Ok(super::Socketeer::new(stream))
        }
    }

    fn is_valid(
        &self,
        conn: &mut Self::Connection,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            if conn.valid {
                Ok(())
            } else {
                Err(Error::Generic("Connection is invalid".into()))
            }
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.valid
    }
}
