FROM rust:slim-bullseye as builder
RUN apt-get -y update && apt-get -y upgrade
RUN apt-get -y install llvm cmake gcc ca-certificates libssl-dev pkg-config
ENV RUSTFLAGS="-C target-feature=-crt-static --cfg=web_sys_unstable_apis"
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo leptos build --release -vv

FROM rust:slim-bullseye as runner
COPY --from=builder /app/target/server/release/aruna_web /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/.env /app/
RUN apt-get -y update && apt-get -y upgrade
RUN apt-get -y install ca-certificates openssl
RUN rustup toolchain install nightly
RUN rustup default nightly
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="aruna_web"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/aruna_web"]