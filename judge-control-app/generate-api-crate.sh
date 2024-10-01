#!/usr/bin/env bash

source ../.env.dev

# Create directories if they don't exist
if [ ! -d "judge-api-server" ]; then
  mkdir judge-api-server
fi
if [ ! -d "backend-api-schema" ]; then
  mkdir backend-api-schema
fi

# Download the API schema
curl "${JUDGE_TO_BACKEND_API}" > judge-api-server/judge-api.yaml
curl "${JUDGE_TO_BACKEND_API}" > backend-api-schema/backend-api.yaml

# Generate the API server crate
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/judge-api-server/judge-api.yaml \
  -g rust-axum \
  -o /local/judge-api-server \
  --additional-properties=packageName=judge_api_server \

# Generate the API schema crate
docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli generate \
  -i /local/backend-api-schema/backend-api.yaml \
  -g rust \
  -o /local/backend-api-schema \
  --additional-properties=packageName=backend_api_schema \