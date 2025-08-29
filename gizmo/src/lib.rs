#![feature(arbitrary_self_types)]
#![feature(never_type)]
#![feature(trait_alias)]
#![feature(type_changing_struct_update)]
#![feature(try_trait_v2)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate lazy_static;

mod client;
// mod conversion;
mod error;
mod network;
mod options;
mod traversal;

pub type GremlinResult<T> = Result<T, Error>;
pub use error::Error;

#[cfg(test)]
mod tests {
    use gizmio::dialects::Janus;
    use gizmio::formats::GraphSON;
    use gizmio::{V3, Vertex};
    use tokio::stream::StreamExt;
    use tokio::task::JoinSet;
    use tokio::tracing::Level;
    use tokio::{console, join};
    use tracing_subscriber::Layer;
    use tracing_subscriber::filter::Targets;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    // pub use client::GremlinClient;
    use crate::client::GremlinClient;
    pub use crate::error::Error;
    use crate::options::ConnectionOptions;
    use crate::traversal::{RemoteTraversalStream, traversal};

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    #[tracing::instrument]
    pub async fn test_janus() -> Result<(), Error> {
        let std_out_filter = Targets::new()
            .with_target("gizmo", Level::TRACE)
            .with_target("hyper_util::client::legacy::pool", Level::INFO);
        // let mut indicatif = IndicatifLayer::new()
        //     .with_max_progress_bars(8, None);
        let console_layer = console::subscriber::spawn();
        let stdout_layer = tracing_subscriber::fmt::layer()
            // .with_writer(indicatif.get_stderr_writer())
            .with_filter(std_out_filter);
        // let indicatif_layer = indicatif.with_filter(IndicatifFilter::new(false));

        tracing_subscriber::registry()
            .with(console_layer)
            .with(stdout_layer)
            // .with(indicatif_layer)
            .init();

        let options = ConnectionOptions::builder()
            .dialect::<Janus>()
            .format::<GraphSON<V3>>()
            .host("0.0.0.0".to_string())
            .port(8182)
            .pool_size(3)
            .build()
            .unwrap();

        let client = GremlinClient::connect(options).await?;
        let g = traversal().with_remote(client);
        let mut set = JoinSet::new();

        for _ in 0..16 {
            let _g = g.clone();
            set.spawn(async move {
                let mut a: RemoteTraversalStream<Vertex> = _g.v(()).iter().await.unwrap();
                while let Some(Ok(thing)) = a.next().await {
                    tracing::info!("{:?}", thing);
                }
            });
        }

        set.join_all().await;

        Ok(())
    }
}

// pub mod prelude {
//     pub use super::*;
//
//     pub use tokio::stream::StreamExt;
//
//     pub use crate::io::{V2, V3, V3g};
//     pub use crate::options::*;
//     pub use crate::{edge, vertex};
//     //
//     pub use crate::process::traversal;
//     pub use crate::process::traversal::__;
//     pub use crate::process::traversal::AsyncTerminator;
//     pub use crate::process::traversal::GraphTraversalSource;
//     pub use crate::process::traversal::traversal;
//
//     pub use crate::*;
// }
