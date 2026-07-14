# --- Build stage ---
FROM rust:1.82-slim AS builder

# duckdb's "bundled" feature compiles DuckDB's C++ core from source,
# which needs cmake + a C++ compiler beyond what rust:slim ships with.
RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    g++ \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache deps separately from source for faster rebuilds on code-only changes.
# NOTE: this still recompiles the bundled DuckDB C++ core on every
# dependency-layer rebuild, since that's tied to Cargo.toml, not src/ --
# this is normal for the bundled feature and just means the *first*
# build is slow (10+ min), not every build.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend

VOLUME ["/app/results"]

EXPOSE 8000
CMD ["/app/backend"]
