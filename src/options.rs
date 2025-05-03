use crate::{Gremlin, GremlinResult, GremlinError};
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::Duration;
use tokio::tracing;
use webpki_roots::TLS_SERVER_ROOTS;

#[derive(Clone, Debug)]
pub struct TlsOptions {
    /// A path to your CA file
    pub authority: Option<PathBuf>,
    /// A path to your private
    pub private_key: Option<String>,
    /// Authentication certificates
    pub auth_certs: Option<String>,
}

impl Default for TlsOptions {
    fn default() -> Self {
        Self {
            authority: None,
            private_key: None,
            auth_certs: None,
        }
    }
}

impl TlsOptions {
    /// Copied pretty directly from https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs
    /// and https://github.com/rustls/tokio-rustls/blob/main/examples/client.rs
    pub(crate) fn config(self) -> Result<tokio::rustls::rustls::ClientConfig, GremlinError> {
        let mut cert_store = rustls::RootCertStore::empty();

        if let Some(ca_file) = self.authority {
            let fd = std::fs::File::open(ca_file)?;
            let mut bufd = BufReader::new(fd);
            let certs = rustls_pemfile::certs(&mut bufd).flatten(); //.collect::<Vec<_>>();

            cert_store.add_parsable_certificates(certs);
        } else {
            cert_store.extend(TLS_SERVER_ROOTS.iter().cloned());
        }

        let base_config =
            tokio::rustls::rustls::ClientConfig::builder().with_root_certificates(cert_store);
        match (&self.private_key, &self.auth_certs) {
            (None, None) => Ok(base_config.with_no_client_auth()),
            (Some(key_file), Some(certs_file)) => {
                let certs = CertificateDer::pem_file_iter(certs_file)?
                    .flat_map(|result| {
                        if let Err(e) = &result {
                            tracing::warn!("{}", e);
                        }
                        result
                    })
                    .collect::<Vec<_>>();
                let private_key = PrivateKeyDer::from_pem_file(key_file)?;
                let config = base_config.with_client_auth_cert(certs, private_key)?;

                Ok(config)
            }
            (None, Some(_)) => {
                tracing::warn!("The certificate file is missing.");
                panic!();
            }
            (Some(_), None) => {
                tracing::warn!("The private key file is missing.");
                panic!();
            }
        }
    }
}

impl<'a, SD: Gremlin> Into<ConnectionOptions<SD>> for &'a str {
    fn into(self) -> ConnectionOptions<SD> {
        let default = ConnectionOptions::<SD>::default();
        ConnectionOptions {
            host: self.to_string(),
            serde: PhantomData::<SD>,
            ..default
        }
    }
}

macro_rules! into_connection_options {
    ($kind:ty) => {
        impl<H, SD> Into<ConnectionOptions<SD>> for (H, $kind)
        where
            H: AsRef<str>,
            SD: Gremlin,
        {
            fn into(self) -> ConnectionOptions<SD> {
                (self.0, self.1 as u16).into()
            }
        }
    };
}

// The assumption here is the compiler's assigned a user-supplied value " ::connect(("", >2749<))"
// some random integer length. Ofc we lose precision casting down but since the standard's
// u16::MAX_SIZE whoever's putting in the numbers is intrinsically limited already. If they do
// put in a huge number, it just won't connect.
into_connection_options!(i16);
into_connection_options!(i32);
into_connection_options!(i64);
impl<H, SD> Into<ConnectionOptions<SD>> for (H, u16)
where
    H: AsRef<str>,
    SD: Gremlin,
{
    fn into(self) -> ConnectionOptions<SD> {
        let default = ConnectionOptions::<SD>::default();
        ConnectionOptions {
            host: String::from(self.0.as_ref()),
            port: self.1,
            serde: PhantomData::<SD>,
            ..default
        }
    }
}

impl<H, P, SD> Into<ConnectionOptions<SD>> for (H, P, SD)
where
    H: AsRef<str>,
    P: Into<u16>,
    SD: Gremlin,
{
    fn into(self) -> ConnectionOptions<SD> {
        let default = ConnectionOptions::<SD>::default();
        ConnectionOptions {
            host: String::from(self.0.as_ref()),
            port: self.1.into(),
            serde: PhantomData::<SD>,
            ..default
        }
    }
}

// impl Into<ConnectionOptions<V3>> for (&str, u16) {
//     fn into(self) -> ConnectionOptions<V3> {
//         let default = ConnectionOptions::<V3>::default();
//         ConnectionOptions {
//             host: String::from(self.0),
//             port: self.1,
//             serde: PhantomData::<V3>,
//             ..default
//         }
//     }
// }
//
// impl Into<ConnectionOptions<V3>> for &str {
//     fn into(self) -> ConnectionOptions<V3> {
//         let default = ConnectionOptions::<V3>::default();
//         ConnectionOptions {
//             host: String::from(self),
//             serde: PhantomData::<V3>,
//             ..default
//         }
//     }
// }

pub struct ConnectionOptionsBuilder<SD: Gremlin>(ConnectionOptions<SD>);

impl<_SD: Gremlin> ConnectionOptionsBuilder<_SD> {
    pub fn host<T>(mut self, host: T) -> Self
    where
        T: Into<String>,
    {
        self.0.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.0.port = port;
        self
    }

    pub fn pool_size(mut self, pool_size: u32) -> Self {
        self.0.pool_size = pool_size;
        self
    }

    /// Only applicable to async client. By default a connection is checked on each return to the pool (None)
    /// This allows setting an interval of how often it is checked on return.
    pub fn pool_healthcheck_interval(
        mut self,
        pool_healthcheck_interval: Option<Duration>,
    ) -> Self {
        self.0.pool_healthcheck_interval = pool_healthcheck_interval;
        self
    }

    /// Both the sync and async pool providers use a default of 30 seconds,
    /// Async pool interprets `None` as no timeout. Sync pool maps `None` to the default value
    pub fn pool_connection_timeout(mut self, pool_connection_timeout: Option<Duration>) -> Self {
        self.0.pool_get_connection_timeout = pool_connection_timeout;
        self
    }

    pub fn build(self) -> ConnectionOptions<_SD> {
        self.0
    }

    pub fn credentials(mut self, username: &str, password: &str) -> Self {
        self.0.credentials = Some(Credentials {
            username: String::from(username),
            password: String::from(password),
        });
        self
    }

    pub fn ssl(mut self, ssl: bool) -> Self {
        self.0.ssl = ssl;
        self
    }

    pub fn tls_options(mut self, options: TlsOptions) -> Self {
        self.0.tls_options = Some(options);
        self
    }

    pub fn websocket_options(mut self, options: WebSocketOptions) -> Self {
        self.0.websocket_options = Some(options);
        self
    }

    pub fn serde<SD: Gremlin>(self, _: SD) -> ConnectionOptionsBuilder<SD> {
        let cloned = ConnectionOptions {
            serde: PhantomData::<SD>,
            ..self.0
        };

        ConnectionOptionsBuilder(cloned)
    }
}

#[derive(Clone, Debug)]
pub struct ConnectionOptions<SD: Gremlin> {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) pool_size: u32,
    pub(crate) pool_healthcheck_interval: Option<Duration>,
    pub(crate) pool_get_connection_timeout: Option<Duration>,
    pub(crate) credentials: Option<Credentials>,
    pub(crate) ssl: bool,
    pub(crate) tls_options: Option<TlsOptions>,
    pub(crate) serde: PhantomData<SD>,
    pub(crate) websocket_options: Option<WebSocketOptions>,
}

#[derive(Clone, Debug)]
pub(crate) struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Clone, Debug)]
pub struct WebSocketOptions {
    /// The maximum size of a message. `None` means no size limit. The default value is 64 MiB.
    pub(crate) max_message_size: Option<usize>,
    /// The maximum size of a single message frame. `None` means no size limit. The limit is for
    /// frame payload NOT including the frame header. The default value is 16 MiB.
    pub(crate) max_frame_size: Option<usize>,
}

impl WebSocketOptions {
    pub fn builder() -> WebSocketOptionsBuilder {
        WebSocketOptionsBuilder(Self::default())
    }
}

impl Default for WebSocketOptions {
    fn default() -> Self {
        Self {
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
        }
    }
}

impl From<WebSocketOptions> for tungstenite::protocol::WebSocketConfig {
    fn from(value: WebSocketOptions) -> Self {
        (&value).into()
    }
}

impl From<&WebSocketOptions> for tungstenite::protocol::WebSocketConfig {
    fn from(value: &WebSocketOptions) -> Self {
        let mut config = tungstenite::protocol::WebSocketConfig::default();
        config.max_message_size = value.max_message_size;
        config.max_frame_size = value.max_frame_size;
        config
    }
}

// impl From<TlsOptions> for std::sync::Arc<>

pub struct WebSocketOptionsBuilder(WebSocketOptions);

impl WebSocketOptionsBuilder {
    pub fn build(self) -> WebSocketOptions {
        self.0
    }

    pub fn max_message_size(mut self, max_message_size: Option<usize>) -> Self {
        self.0.max_message_size = max_message_size;
        self
    }

    pub fn max_frame_size(mut self, max_frame_size: Option<usize>) -> Self {
        self.0.max_frame_size = max_frame_size;
        self
    }
}

impl<SD: Gremlin> Default for ConnectionOptions<SD> {
    fn default() -> ConnectionOptions<SD> {
        ConnectionOptions {
            host: String::from("localhost"),
            port: 8182,
            pool_size: 10,
            pool_get_connection_timeout: Some(Duration::from_secs(30)),
            pool_healthcheck_interval: None,
            credentials: None,
            ssl: false,
            tls_options: None,
            serde: PhantomData::<SD>,
            websocket_options: None,
        }
    }
}

impl ConnectionOptions<()> {
    pub fn builder() -> ConnectionOptionsBuilder<()> {
        ConnectionOptionsBuilder(ConnectionOptions::default())
    }
}

impl<SD: Gremlin> ConnectionOptions<SD> {
    pub fn websocket_url(&self) -> String {
        let protocol = if self.ssl { "wss" } else { "ws" };
        format!("{}://{}:{}/gremlin", protocol, self.host, self.port)
    }
}
