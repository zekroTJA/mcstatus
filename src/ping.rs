use anyhow::Result;
use craftping::Response;
use tokio::net::TcpStream;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::{ResolveError, ResolveErrorKind},
    AsyncResolver,
};

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

pub async fn ping(host: &str) -> Result<Response> {
    let (host, port) = resolve(host)
        .await?
        .unwrap_or_else(|| (host.to_string(), 25565));
    dbg!(&host, port);
    let mut stream = TcpStream::connect((host.as_str(), port)).await?;
    let pong = craftping::tokio::ping(&mut stream, &host, port).await?;
    Ok(pong)
}
