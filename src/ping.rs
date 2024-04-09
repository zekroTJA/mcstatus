use std::time::Duration;

use anyhow::Result;
use elytra_ping::{ping_or_timeout, JavaServerInfo};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::{ResolveError, ResolveErrorKind},
    AsyncResolver,
};

pub type Response = JavaServerInfo;

#[inline]
fn is_not_found(err: &ResolveError) -> bool {
    matches!(
        err.kind(),
        ResolveErrorKind::NoRecordsFound {
            query: _,
            soa: _,
            negative_ttl: _,
            response_code: _,
            trusted: _
        }
    )
}

async fn resolve(addr: &str) -> Result<Option<(String, u16)>> {
    let addr = match addr.starts_with("_minecraft._tcp.") {
        true => addr.to_string(),
        _ => format!("_minecraft._tcp.{addr}"),
    };

    let res = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    match res.srv_lookup(addr).await {
        Ok(v) => Ok(v.iter().next().map(|srv| {
            (
                srv.target().to_string().trim_end_matches('.').to_string(),
                srv.port(),
            )
        })),
        Err(e) if is_not_found(&e) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn ping(host: &str, port: Option<u16>) -> Result<Response> {
    let (host, port) = match port {
        Some(port) => (host.to_string(), port),
        None => resolve(host)
            .await?
            .unwrap_or_else(|| (host.to_string(), 25565)),
    };

    // TODO: Add some cahcing for host and port?
    let (pong, _) = ping_or_timeout((host.to_string(), port), Duration::from_secs(1)).await?;
    Ok(pong)
}
