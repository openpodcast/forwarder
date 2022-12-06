# Dockerize Wrangler Rust application
FROM rust:latest

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    libssl-dev \
    pkg-config \
    binaryen # wasm-opt

# Install Wrangler 2.0 (via nodejs)
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g wrangler
RUN wrangler --version

WORKDIR /app

# Copy `Cargo.toml` and `Cargo.lock` to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy source code (lib.rs) to cache dependencies
RUN mkdir src
RUN touch src/lib.rs

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source code
COPY . .

# Build application
# RUN cargo build --release

RUN cargo install -q worker-build && worker-build --release

# Set WRANGLER_SEND_METRICS to false to disable telemetry
ENV WRANGLER_SEND_METRICS false

ENTRYPOINT ["wrangler"]
CMD ["dev", "--local"]
