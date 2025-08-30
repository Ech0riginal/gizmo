use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::time::Duration;

use derive_builder::Builder;
use gizmio::{Dialect, Format};
use rustls_pki_types::pem::PemObject;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tokio::tracing;
use webpki_roots::TLS_SERVER_ROOTS;

use crate::Error;
use crate::client::Supports;

#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned")]
pub struct ConnectionOptions<D, F> {
    #[allow(unused)]
    #[builder(setter(skip))]
    pub(crate) dialect: PhantomData<D>,
    #[allow(unused)]
    #[builder(setter(skip))]
    pub(crate) format: PhantomData<F>,
    #[builder(default = "Self::default_host()")]
    pub(crate) host: String,
    #[builder(default = "Self::default_port()")]
    pub(crate) port: u16,
    #[builder(default = "Self::default_poolsize()")]
    pub(crate) pool_size: u32,
    #[builder(default = "Self::default_idletimeout()")]
    pub(crate) idle_timeout: Duration,
    #[builder(default = "Self::default_timeout()")]
    pub(crate) connection_timeout: Duration,
    #[builder(default = "Self::default_credentials()")]
    pub(crate) credentials: Option<Credentials>,
    #[builder(default = "Self::default_ssl()")]
    pub(crate) ssl: bool,
    #[builder(default = "Self::default_tlsoptions()")]
    pub(crate) tls_options: Option<TlsOptions>,
    #[builder(default = "Self::default_wsoptions()")]
    pub(crate) websocket_options: Option<WebSocketOptions>,
}

#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned")]
pub(crate) struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Clone, Debug, Builder)]
#[builder(pattern = "owned")]
pub struct TlsOptions {
    /// A path to your CA file
    pub authority: Option<PathBuf>,
    /// A path to your private
    pub private_key: Option<String>,
    /// Authentication certificates
    pub auth_certs: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WebSocketOptions {
    /// The maximum size of a message. `None` means no size limit. The default value is 64 MiB.
    pub(crate) max_message_size: Option<usize>,
    /// The maximum size of a single message frame. `None` means no size limit. The limit is for
    /// frame payload NOT including the frame header. The default value is 16 MiB.
    pub(crate) max_frame_size: Option<usize>,
}

pub struct WebSocketOptionsBuilder(WebSocketOptions);

impl ConnectionOptions<(), ()> {
    pub fn builder() -> ConnectionOptionsBuilder<(), ()> {
        ConnectionOptionsBuilder::create_empty()
    }
}

impl<D: Dialect, F: Supports<D>> ConnectionOptions<D, F> {
    pub fn websocket_url(&self) -> String {
        let protocol = if self.ssl { "wss" } else { "ws" };
        format!("{}://{}:{}/gremlin", protocol, self.host, self.port)
    }
}

impl<F_, D_> ConnectionOptionsBuilder<D_, F_> {
    pub fn dialect<D: Dialect>(self) -> ConnectionOptionsBuilder<D, F_> {
        ConnectionOptionsBuilder {
            dialect: PhantomData::<PhantomData<D>>::default(),
            ..self
        }
    }
    pub fn format<F: Format>(self) -> ConnectionOptionsBuilder<D_, F> {
        ConnectionOptionsBuilder {
            format: PhantomData::<PhantomData<F>>::default(),
            ..self
        }
    }
    fn default_host() -> String {
        "127.0.0.1".into()
    }
    fn default_port() -> u16 {
        8182
    }
    fn default_poolsize() -> u32 {
        8
    }
    fn default_idletimeout() -> Duration {
        Duration::from_secs(60)
    }
    fn default_timeout() -> Duration {
        Duration::from_secs(30)
    }
    fn default_credentials() -> Option<Credentials> {
        None
    }
    fn default_ssl() -> bool {
        false
    }
    fn default_tlsoptions() -> Option<TlsOptions> {
        None
    }
    fn default_wsoptions() -> Option<WebSocketOptions> {
        None
    }
}
impl TlsOptions {
    /// Copied pretty directly from https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs
    /// and https://github.com/rustls/tokio-rustls/blob/main/examples/client.rs
    pub(crate) fn config(self) -> Result<rustls::ClientConfig, Error> {
        let mut cert_store = rustls::RootCertStore::empty();

        if let Some(ca_file) = self.authority {
            let fd = std::fs::File::open(ca_file)?;
            let mut bufd = BufReader::new(fd);
            let certs = rustls_pemfile::certs(&mut bufd).flatten(); //.collect::<Vec<_>>();

            cert_store.add_parsable_certificates(certs);
        } else {
            cert_store.extend(TLS_SERVER_ROOTS.iter().cloned());
        }

        let base_config = rustls::ClientConfig::builder().with_root_certificates(cert_store);
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
