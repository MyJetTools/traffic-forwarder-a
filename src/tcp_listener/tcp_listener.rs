use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf},
    net::{TcpListener, TcpStream},
};

use crate::{app::AppContext, tcp_listener::TcpConnection};

pub struct ServiceTcpListener {
    addr: SocketAddr,
    remote_host: String,
    buffer_size: usize,
    app: Arc<AppContext>,
}

impl ServiceTcpListener {
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

    pub fn start(&self) {
        tokio::spawn(listen_to_sockets(
            self.app.clone(),
            self.addr.clone(),
            self.remote_host.clone(),
            self.buffer_size,
        ));
    }
}

async fn listen_to_sockets(
    app: Arc<AppContext>,
    addr: SocketAddr,
    remote_host: String,
    buffer_size: usize,
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
        }

        socket_id += 1;
        let (read_stream, write_stream) = tokio::io::split(tcp_stream);

        let tcp_connection = Arc::new(TcpConnection::new(socket_id, write_stream));

        let tunnel_connection = app
            .tunnel_tcp_connection
            .new_tcp_connection(&tcp_connection)
            .await;

        if tunnel_connection.is_none() {
            println!(
                "Tunnel connection went down. Dropping connection {} with id {}",
                socket_addr, socket_id
            );
            tcp_connection.disconnect();
        }

        let app_to_spawn = app.clone();

        tokio::spawn(read_loop(
            app_to_spawn,
            read_stream,
            buffer_size,
            tcp_connection,
        ));
    }
}

async fn read_loop(
    app: Arc<AppContext>,
    mut read_stream: ReadHalf<TcpStream>,
    buffer_size: usize,
    tcp_connection: Arc<TcpConnection>,
) {
    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

    loop {
        unsafe {
            buffer.set_len(buffer_size);
        }

        match read_stream.read(&mut buffer).await {
            Ok(read_amount) => {
                if read_amount == 0 {
                    println!(
                        "Socket {} got 0 bytes. Stopping read_stream",
                        tcp_connection.id,
                    );
                    break;
                }

                if !app
                    .tunnel_tcp_connection
                    .send_payload_to_tunnel(tcp_connection.id, &buffer[..read_amount])
                    .await
                {
                    println!(
                        "Tunnel has not connection anymore. Stopping read_stream of socket {}",
                        tcp_connection.id,
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

    app.tunnel_tcp_connection
        .disconnect_tcp_connection(tcp_connection.id)
        .await;
}
