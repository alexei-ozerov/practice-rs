#-------------------#
# Cargo Build Stage #
#-------------------#
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/ms

# Build Deps, Cache & Clean Up
COPY logging-ms/Cargo.toml Cargo.toml
RUN mkdir src/ && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/ms*

# Build Application
COPY logging-ms/ .
RUN cargo build --release
RUN cargo install --path .

#--------------------------#
# Microservice Build Stage #
#--------------------------#
FROM debian:stretch-slim

# Add Underprivileged User
RUN addgroup -uid 1000 ms
RUN useradd -s /bin/bash -g ms -G ms -u 1000 ms

# Pre-Requisite For Install Postgres Client On Stretch-Slim
RUN mkdir -p /usr/share/man/man1 /usr/share/man/man7
# Install Dependencies For DIESEL
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install postgresql postgresql-client -y
COPY --from=builder /usr/local/cargo/bin/ms /usr/local/bin/ms

# Run MS with Logs Enabled As MS User
WORKDIR /usr/local/bin/
RUN chown ms:ms ms
USER ms
ENV RUST_LOG=DEBUG
CMD ["ms"]