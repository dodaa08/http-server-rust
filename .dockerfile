# Stage 1: Prepare dependencies with cargo chef
FROM rust:1.82 AS chef
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build dependencies
FROM rust:1.82 AS builder
WORKDIR /app
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin hello-world

# Stage 3: Create runtime image
FROM debian:bookworm-slim AS runner
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/hello-world /app/hello-world
EXPOSE 3000
CMD ["/app/hello-world"]