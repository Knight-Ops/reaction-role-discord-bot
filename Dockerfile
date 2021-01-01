# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

# RUN apt-get install musl-tools -y

# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-gnu

RUN rm -f target/x86_64-unknown-linux-gnu/release/deps/reaction-role-discord-bot*

COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-gnu

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM ubuntu:latest

RUN addgroup --gid 1000 myapp

RUN adduser --disabled-password --shell /bin/sh --uid 1000 --ingroup myapp myapp

WORKDIR /home/myapp/bin/

COPY --from=cargo-build /usr/src/myapp/target/x86_64-unknown-linux-gnu/release/reaction-role-discord-bot .

RUN chown myapp:myapp reaction-role-discord-bot

USER myapp

CMD ["./reaction-role-discord-bot"]