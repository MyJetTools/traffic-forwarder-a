use std::{collections::HashMap, sync::Arc};

use super::TargetTcpConnection;

pub struct TargetTcpListenerConnections {
    connections: HashMap<u32, Arc<TargetTcpConnection>>,
}

impl TargetTcpListenerConnections {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn add(&mut self, connection: Arc<TargetTcpConnection>) {
        self.connections.insert(connection.id, connection);
    }

    pub fn get(&self, connection_id: u32) -> Option<Arc<TargetTcpConnection>> {
        let result = self.connections.get(&connection_id)?;
        Some(result.clone())
    }

    pub fn remove(&mut self, connection_id: u32) -> Option<Arc<TargetTcpConnection>> {
        self.connections.remove(&connection_id)
    }

    pub fn remove_all(self) -> HashMap<u32, Arc<TargetTcpConnection>> {
        self.connections
    }
}
