FROM python:3.12.8-slim AS venv-builder
RUN apt-get update && \
    apt-get install -y curl clang
RUN curl -sSf https://rye.astral.sh/get | RYE_INSTALL_OPTION="--yes" bash
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.rye/shims:/root/.cargo/bin:${PATH}"
COPY . /app
WORKDIR /app
RUN --mount=type=cache,target=/root/.cache,sharing=locked \
    --mount=type=cache,target=/root/.cargo/registry,sharing=locked \
    rye sync --pyproject pylib/traopy_util/pyproject.toml

FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y g++ sudo

COPY ./pylib/example/languages.json /trao/languages.json
ENV TRAOJUDGE_LANGUAGES_JSON=/trao/languages.json
COPY --from=venv-builder /app/pylib/traopy_util /app/pylib/traopy_util
COPY --from=venv-builder /root/.rye/py /root/.rye/py
ENV PATH="/app/pylib/traopy_util/.venv/bin:${PATH}"
RUN ln -sf /app/pylib/traopy_util/.venv/bin/python3 /bin/python3-traopy
