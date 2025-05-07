use futures::SinkExt;
use tokio::stream::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::SendError;
use tungstenite::Message;
use super::WSStream;

pub struct Stream(UnboundedReceiver<Message>);

#[derive(Clone)]
pub struct Sink(UnboundedSender<Message>);

impl Sink {
    pub fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
        self.0.send(msg)
    }
}

impl Into<Sink> for futures::stream::SplitSink<WSStream, Message> {
    fn into(mut self) -> Sink {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = self.send(msg).await {
                    tracing::warn!("Error sending message: {:?}", e);
                    break;
                }
            }
        });

        Sink(tx)
    }
}

impl Stream {
    pub fn recv(&mut self) -> impl Future<Output = Option<Message>> {
        self.0.recv()
    }
}

impl Into<Stream> for futures::stream::SplitStream<WSStream> {
    fn into(mut self) -> Stream {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(result) = self.next().await {
                match result {
                    Ok(msg) => match tx.send(msg) {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::warn!("Error reading message: {:?}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Error reading message: {:?}", e);
                        break;
                    }
                }
            }
        });

        Stream(rx)
    }
}
