FROM rust:1-slim
WORKDIR app
COPY . .
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build; \
    cp target/debug/rocket-surrealdb main
CMD main
