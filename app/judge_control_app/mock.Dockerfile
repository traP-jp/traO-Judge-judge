FROM rust:bookworm AS builder
WORKDIR /app

COPY . /app

RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=cache,target=/var/cache/cargo,sharing=locked \
    cargo build --release --bin mock

FROM debian:bookworm-slim
EXPOSE 8080
ENV TRAOJUDGE_PROBLEM_REGISTRY_DIR=/trao/judge_control/registry
ENV TRAOJUDGE_JOBAPI_CACHE_DIR=/trao/judge_control/cache
ENV TRAOJUDGE_GRPC_SERVICE_PORT=8080
ENV RUST_LOG=debug
COPY --from=builder /app/target/release/mock /app/target/release/mock
CMD ["/app/target/release/mock"]
