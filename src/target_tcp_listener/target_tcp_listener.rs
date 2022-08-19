use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf},
    net::{TcpListener, TcpStream},
};

use crate::{app::AppContext, target_tcp_listener::TargetTcpConnection};

use super::TargetTcpCallbacks;

pub struct TargetTcpListener {
    addr: SocketAddr,
    pub remote_host: String,
    buffer_size: usize,
    app: Arc<AppContext>,
}

impl TargetTcpListener {
    pub fn new(
        app: Arc<AppContext>,
        addr: SocketAddr,
        remote_host: String,
        buffer_size: usize,
    ) -> Self {
        Self {
            addr,
            remote_host,
            buffer_size,
            app,
        }
    }

    pub fn start(&self, callbacks: Arc<TargetTcpCallbacks>) {
        tokio::spawn(listen_to_sockets(
            self.app.clone(),
            self.addr.clone(),
            self.remote_host.clone(),
            self.buffer_size,
            callbacks,
        ));
    }
}

async fn listen_to_sockets(
    app: Arc<AppContext>,
    addr: SocketAddr,
    remote_host: String,
    buffer_size: usize,
    callbacks: Arc<TargetTcpCallbacks>,
) {
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Started port {} connected to {}", addr, remote_host);

    let mut socket_id: u32 = 0;

    loop {
        let accept_result = listener.accept().await;

        if let Err(err) = &accept_result {
            println!("Error accepting connection to port{}. Err:{}", addr, err);
            continue;
        }

        let (mut tcp_stream, socket_addr) = accept_result.unwrap();

        if !app.tunnel_tcp_connection.is_connection_established() {
            println!(
                "Tunnel connection is not established. Dropping connection {}",
                socket_addr
            );
            let _shut_down_result = tcp_stream.shutdown().await;
            continue;
        }

        socket_id += 1;
        let (read_stream, write_stream) = tokio::io::split(tcp_stream);

        let target_tcp_connection = Arc::new(TargetTcpConnection::new(socket_id, write_stream));

        if !callbacks.on_new_connection(&target_tcp_connection).await {
            println!(
                "Tunnel connection is dropped. Dropping target connection {}",
                socket_addr
            );
            continue;
        }

        tokio::spawn(read_loop(
            read_stream,
            buffer_size,
            callbacks.clone(),
            target_tcp_connection,
        ));
    }
}

async fn read_loop(
    mut read_stream: ReadHalf<TcpStream>,
    buffer_size: usize,
    callbacks: Arc<TargetTcpCallbacks>,
    target_tcp_connection: Arc<TargetTcpConnection>,
) {
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);
    unsafe {
        buffer.set_len(buffer_size);
    }

    loop {
        match read_stream.read(&mut buffer).await {
            Ok(read_amount) => {
                if read_amount == 0 {
                    break;
                }

                if !callbacks
                    .on_new_payload(&target_tcp_connection, buffer[..read_amount].to_vec())
                    .await
                {
                    println!(
                        "Tunnel is disconnected. Stopping read_stream for target connection {}",
                        target_tcp_connection.id
                    );
                    break;
                }
            }
            Err(err) => {
                println!(
                    "Error reading from socket. Err:{}. Stopping read_stream",
                    err
                );
                break;
            }
        }
    }

    callbacks.on_new_disconnection(target_tcp_connection).await;
}
