FROM rust:1.86.0-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src


# Baue Abhängigkeiten
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Final Stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y ffmpeg && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY default.config.toml ./

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/hsl_service ./

ENV RUST_LOG=info
ENV ISM_MODE=production

EXPOSE 5555

CMD ["./hsl_service"]