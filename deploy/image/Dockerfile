# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest AS cargo-build

RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml ./

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/reaction-role-discord-bot*

COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM gcr.io/distroless/static-debian12

COPY --from=cargo-build \
    /usr/src/app/target/x86_64-unknown-linux-musl/release/reaction-role-discord-bot \
    /app/

WORKDIR /app
USER nonroot:nonroot

CMD ["./reaction-role-discord-bot"]
