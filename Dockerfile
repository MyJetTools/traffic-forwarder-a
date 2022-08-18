FROM rust:slim
COPY ./target/release/traffic-forwarder-a ./target/release/traffic-forwarder-a 
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/traffic-forwarder-a"]