use bollard::service::PortBinding;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub ip: String,
    pub port: u32,
    pub name: String,
    pub owner: String,
}

impl Server {
    pub fn new(ip: String, port: u32, name: String, owner: String) -> Self {
        Self {
            id: Ulid::new().to_string(),
            ip,
            port,
            name,
            owner,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Application {
    pub id: String,
    pub image: String,
    pub alias: String,
    pub owner: String,
    pub server: String,
    pub ports: Option<Vec<PortBinding>>,
}
