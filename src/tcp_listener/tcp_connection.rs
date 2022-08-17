use std::sync::atomic::AtomicBool;
use std::time::Duration;

use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct TcpConnection {
    pub id: u32,
    sender: UnboundedSender<Option<Vec<u8>>>,
    disconnected: AtomicBool,
}

impl TcpConnection {
    pub fn new(id: u32, tcp_stream: WriteHalf<TcpStream>) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(tcp_send_loop(id, receiver, tcp_stream));

        Self {
            id,
            sender,
            disconnected: AtomicBool::new(false),
        }
    }

    pub fn send_payload(&self, payload: Vec<u8>) {
        let _ = self.sender.send(Some(payload));
    }

    pub fn disconnect(&self) {
        let before_was_connected = self
            .disconnected
            .swap(true, std::sync::atomic::Ordering::SeqCst);

        if before_was_connected {
            let _ = self.sender.send(None);
        }
    }
}

async fn tcp_send_loop(
    id: u32,
    mut receiver: UnboundedReceiver<Option<Vec<u8>>>,
    mut tcp_stream: WriteHalf<TcpStream>,
) {
    let send_timeout = Duration::from_secs(15);
    while let Some(next) = receiver.recv().await {
        match next {
            Some(payload) => {
                let future = tcp_stream.write_all(payload.as_slice());

                let result = tokio::time::timeout(send_timeout, future).await;

                if result.is_err() {
                    println!("TcpConnection:{}: send timeout", id);
                    break;
                }

                let result = result.unwrap();

                if let Err(err) = result {
                    println!("TcpConnection:{} has error {}", id, err);
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    let _ = tcp_stream.shutdown().await;
}
