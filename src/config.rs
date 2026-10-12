use crate::constants::DEFAULT_SERVER_ADDRESS;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: DEFAULT_SERVER_ADDRESS.to_owned(),
        }
    }
}
