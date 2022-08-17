use std::{collections::HashMap, sync::Arc};

use super::TcpConnection;

pub struct TcpListenerConnections {
    connections: HashMap<u32, Arc<TcpConnection>>,
}

impl TcpListenerConnections {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn add(&mut self, connection: Arc<TcpConnection>) {
        self.connections.insert(connection.id, connection);
    }

    pub fn get(&self, connection_id: u32) -> Option<Arc<TcpConnection>> {
        let result = self.connections.get(&connection_id)?;
        Some(result.clone())
    }

    pub fn remove(&mut self, connection_id: u32) -> Option<Arc<TcpConnection>> {
        self.connections.remove(&connection_id)
    }

    pub fn remove_all(self) -> HashMap<u32, Arc<TcpConnection>> {
        self.connections
    }
}
