#!/usr/bin/env
source .env.dev
generate-api-crate () {
    rm -rf "$PWD"/"$1"
    mkdir -p "$PWD"/"$1"
    docker run --rm \
        -v "$PWD":/local openapitools/openapi-generator-cli generate \
        -i "$2" \
        -g rust \
        -o /local/"$1"
    sudo chown -R "$(stat -c %U .)":"$(stat -c %G .)" "$PWD"/"$1"
}

generate-api-crate backend-api https://raw.githubusercontent.com/traP-jp/traO-Judge-docs/"$API_VERSION"/api/backend/to_judge.yaml
generate-api-crate judge-api https://raw.githubusercontent.com/traP-jp/traO-Judge-docs/"$API_VERSION"/api/judge/to_backend.yaml

