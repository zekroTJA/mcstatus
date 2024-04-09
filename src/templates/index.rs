use crate::config;
use askama::Template;

struct Server {
    name: String,
    host: String,
    port: Option<u16>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    servers: Vec<Server>,
}

impl From<&config::Server> for Server {
    fn from(value: &config::Server) -> Self {
        let (host, port) = match &value.address {
            config::ServerAddress::HostPort { host, port } => (host.clone(), Some(port.get())),
            config::ServerAddress::Address { address } => (address.clone(), None),
        };

        Self {
            name: value.name.clone(),
            host,
            port,
        }
    }
}

impl From<config::Config> for Index {
    fn from(value: config::Config) -> Self {
        Self {
            servers: value.servers.iter().map(|s| s.into()).collect(),
        }
    }
}
