# syntax = docker/dockerfile:1

# Stage 1: Build the Yew WASM frontend
FROM rust:latest AS frontend_builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown

# Download and install precompiled Trunk
RUN curl -sL https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin

# Install standalone tailwindcss CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-linux-x64 && \
    chmod +x tailwindcss-linux-x64 && \
    mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

COPY . .
WORKDIR /app/frontend
RUN trunk --version
RUN tailwindcss --help
RUN trunk build --release

# Stage 2: Build the Axum server backend
FROM rust:latest AS backend_builder
WORKDIR /app
COPY . .
# Copy built static files from frontend stage into dist
COPY --from=frontend_builder /app/frontend/dist /app/dist
RUN cargo build --release --bin server

# Stage 3: Slim Runner
FROM debian:bookworm-slim AS runner
LABEL org.opencontainers.image.source="https://github.com/UberMetroid/rustle"
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy compiled server binary and compiled static assets
COPY --from=backend_builder /app/target/release/server /app/server
COPY --from=frontend_builder /app/frontend/dist /app/dist

# Run as non-root user for security
RUN groupadd -g 10001 appgroup && \
    useradd -u 10001 -g appgroup -m -s /sbin/nologin appuser
USER appuser

EXPOSE 4409
ENV PORT=4409
CMD ["/app/server"]
