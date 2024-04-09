FROM rust:alpine AS build

RUN apk add nodejs npm musl-dev

WORKDIR /build

COPY css/ css/
COPY src/ src/
COPY templates/ templates/
COPY Cargo.toml .
COPY Cargo.lock .
COPY package.json .
COPY package-lock.json .
COPY tailwind.config.js .

RUN cargo build --release

# ------------------------------------------------

FROM alpine:latest

WORKDIR /app

COPY --from=build /build/target/release/mcstatus /bin/mcstatus
COPY --from=build /build/css/ css/

ENV MCSTATUS_ADDRESS="0.0.0.0:80"
EXPOSE 80

ENTRYPOINT [ "/bin/mcstatus" ]