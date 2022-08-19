use std::sync::atomic::AtomicBool;
use std::time::Duration;

use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub enum SendToConnectionsEvent {
    Payload(Vec<u8>),
    Disconnected,
}

pub struct TargetTcpConnection {
    pub id: u32,
    sender: UnboundedSender<SendToConnectionsEvent>,
    disconnected: AtomicBool,
}

impl TargetTcpConnection {
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
        let _ = self.sender.send(SendToConnectionsEvent::Payload(payload));
    }

    pub fn disconnect(&self) {
        let before_was_disconnected = self
            .disconnected
            .swap(true, std::sync::atomic::Ordering::SeqCst);

        if !before_was_disconnected {
            let _ = self.sender.send(SendToConnectionsEvent::Disconnected);
        }
    }
}

async fn tcp_send_loop(
    id: u32,
    mut receiver: UnboundedReceiver<SendToConnectionsEvent>,
    mut tcp_stream: WriteHalf<TcpStream>,
) {
    while let Some(next) = receiver.recv().await {
        match next {
            SendToConnectionsEvent::Payload(payload) => {
                send_payload(&mut tcp_stream, payload, id).await;
            }

            SendToConnectionsEvent::Disconnected => {
                break;
            }
        }
    }

    let _ = tcp_stream.shutdown().await;
}

const SEND_TIMEOUT: Duration = Duration::from_secs(15);
async fn send_payload(tcp_stream: &mut WriteHalf<TcpStream>, payload: Vec<u8>, id: u32) -> bool {
    let future = tcp_stream.write_all(payload.as_slice());

    let result = tokio::time::timeout(SEND_TIMEOUT, future).await;

    if result.is_err() {
        println!("TcpConnection:{}: send timeout", id);
        return false;
    }

    let result = result.unwrap();

    if let Err(err) = result {
        println!("TcpConnection:{} has error {}", id, err);
        return false;
    }

    true
}
