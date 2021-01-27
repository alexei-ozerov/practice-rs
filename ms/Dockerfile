# Define Build Layer
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src

# 1a: Update + Deps
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download & Compile Rust Deps
RUN USER=root cargo new --bin logging-ms
WORKDIR /usr/src/logging-ms
COPY logging-ms/Cargo.toml logging-ms/Cargo.lock ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 1c: Build Binary
COPY logging-ms/src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN ls -la
RUN ls -la target/release
RUN ls -la /usr/local/cargo/bin/

# 2: Copy & Deploy Binary To New Image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/ms .
USER 1000
CMD ["./logging-ms"]
