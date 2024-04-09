use crate::ping::Response;
use askama::Template;

#[derive(Template)]
#[template(path = "server.html")]
pub struct Server {
    version: Option<String>,
    players: Option<(u32, u32)>,
    player_list: Vec<String>,
    favicon: Option<String>,
}

impl From<Response> for Server {
    fn from(value: Response) -> Self {
        Self {
            version: value.version.as_ref().map(|v| v.name.clone()),
            players: value.players.as_ref().map(|p| (p.online, p.max)),
            player_list: value
                .players
                .as_ref()
                .and_then(|p| p.sample.as_ref())
                .map(|s| s.iter().cloned().filter_map(|ps| ps.name).collect())
                .unwrap_or_default(),
            favicon: value.favicon,
        }
    }
}
