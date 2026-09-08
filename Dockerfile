# ---------- Rust build ----------
FROM rust:1.89-bookworm AS rust-builder

WORKDIR /solver

COPY solver/Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release --bin solver

COPY solver/ .
RUN cargo build --release --bin solver


# ---------- Python application ----------
FROM python:3.12-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        gcc \
        libssl3 \
        libmariadb-dev \
    && rm -rf /var/lib/apt/lists/*

# Python dependencies
COPY server/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Application
COPY server/ .

# Compiled Rust solver
COPY --from=rust-builder /solver/target/release/solver /usr/local/bin/solver

RUN chmod +x /usr/local/bin/solver

# Run as fshare
USER 1001:1001

EXPOSE 5000

CMD ["flask", "run", "--host=0.0.0.0", "--port=5000"]