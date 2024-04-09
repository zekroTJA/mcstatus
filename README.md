# mcstatus

Very simple Minecraft server status page. This project is basically just an exploration of HTMX in Rust with [askama](https://crates.io/crates/askama), [axum](https://crates.io/crates/axum) and [tailwind](https://tailwindcss.com).

## Setup

If you want to set this up yourself, simply use the provided [Docker image](https://github.com/zekroTJA/mcstatus/pkgs/container/mcstatus).

Servers can be specified in a config file, which is mounted via a volume to the container. The location of the config in the container is passed as the first command line parameter.

**Example**

> `mcstatus/config.toml`
```toml
[[servers]]
name = "Vanilla Server"
address = "mc.example.com"

[[servers]]
name = "Modded Server"
host = "modded.example.com"
port = 25678
```

> `docker-compose.yml`
```yaml
services:
  mcstatus:
    image: "ghcr.io/zekrotja/mcstatus:latest"
    command: "/var/config.toml"
    volumes:
      - "./mcstatus/config.toml:/var/config.toml:ro"
    restart: unless-stopped
```

By default, the service is exposed on port `80` in the Docker container. But you can customize the address and port via the `address` config property.