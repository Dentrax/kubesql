FROM rust:1.51.0-alpine3.13@sha256:7e00c2c4048ec8eee07f5fa34c1c2098f21fe342bbe37105f11eb2c7d04b08e4 as builder

WORKDIR /usr/src

RUN apk add --no-cache musl-dev openssl-dev libressl libressl-dev pkgconfig perl make

RUN USER=root cargo new --bin kubesql

WORKDIR /usr/src/kubesql

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo install --target x86_64-unknown-linux-musl --path .

COPY src ./src

RUN cargo build --release
RUN cargo install --path .

FROM gcr.io/distroless/static:nonroot

COPY --from=builder /usr/local/cargo/bin/kubesql /kubesql

USER nonroot:nonroot

ENTRYPOINT ["/kubesql"]
