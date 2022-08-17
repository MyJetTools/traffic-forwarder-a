use std::net::SocketAddr;

pub struct ServiceSettings {
    pub port: u16,
    pub remote_host: String,
}

pub struct SettingsModel {
    pub services: Vec<ServiceSettings>,
    pub tunnel_hand_shake_phrase: String,
}

impl SettingsModel {
    pub fn load() -> Self {
        todo!("Implement")
    }
    pub fn get_tunnel_addr(&self) -> SocketAddr {
        todo!("Implement");
    }
}
