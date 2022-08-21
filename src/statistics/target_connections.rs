use std::{collections::HashMap, sync::atomic::AtomicU16};

use crate::settings_model::SettingsModel;

pub struct TargetConnectionsMetrics {
    pub listening_port: u16,
    pub remote_host: String,
    pub connections: u16,
}

pub struct TargetConnectionStatistics {
    pub remote_host: String,
    pub connections: AtomicU16,
}

pub struct TargetConnections {
    pub target_services: HashMap<u16, TargetConnectionStatistics>,
}

impl TargetConnections {
    pub fn new(settings_model: &SettingsModel) -> Self {
        let mut target_services = HashMap::new();

        for service in settings_model.get_serives() {
            target_services.insert(
                service.port,
                TargetConnectionStatistics {
                    remote_host: service.remote_host,
                    connections: AtomicU16::new(0),
                },
            );
        }
        Self { target_services }
    }

    pub fn get_metrics(&self) -> Vec<TargetConnectionsMetrics> {
        let mut result = Vec::with_capacity(self.target_services.len());

        for (port, statistics) in self.target_services.iter() {
            result.push(TargetConnectionsMetrics {
                listening_port: *port,
                remote_host: statistics.remote_host.clone(),
                connections: statistics
                    .connections
                    .load(std::sync::atomic::Ordering::Relaxed),
            });
        }

        result
    }

    pub fn new_connection(&self, port: u16) {
        if let Some(item) = self.target_services.get(&port) {
            item.connections
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub fn new_disconnection(&self, port: u16) {
        if let Some(item) = self.target_services.get(&port) {
            item.connections
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
