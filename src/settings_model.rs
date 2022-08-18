use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;

pub struct ServiceSettings {
    pub port: u16,
    pub remote_host: String,
}

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "Services")]
    pub services: Vec<String>,
    #[serde(rename = "TunnelHandShakePhrase")]
    pub tunnel_hand_shake_phrase: String,
    #[serde(rename = "Port")]
    pub port: u16,
}

impl SettingsModel {
    pub fn get_tunnel_addr(&self) -> SocketAddr {
        SocketAddr::from(([0, 0, 0, 0], self.port))
    }

    pub fn get_serives(&self) -> Vec<ServiceSettings> {
        let mut result = Vec::new();
        for service in &self.services {
            let mut port = None;
            let mut remote_host = None;
            let mut i = 0;
            for part in service.split('>') {
                match i {
                    0 => {
                        port = Some(part.parse::<u16>().unwrap());
                    }
                    1 => remote_host = Some(part.to_string()),
                    _ => {}
                }

                i += 1;
            }

            result.push(ServiceSettings {
                port: port.unwrap(),
                remote_host: remote_host.unwrap(),
            });
        }

        result
    }
}
