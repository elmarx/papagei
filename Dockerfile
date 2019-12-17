FROM rust:latest as builder

WORKDIR /usr/src

# pre-compile dependencies for improved cacheability
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    # release-build (for install)
    cargo build --release --locked && \
    rm -r src

COPY src src
RUN cargo install --locked --path . --root /usr/local

FROM debian:latest
COPY --from=builder /usr/local/bin/* /usr/local/bin/
EXPOSE 8080
CMD http_echo
