FROM swaggerapi/swagger-ui:v5.3.1
COPY docs/openapi.yaml /docs/openapi.yaml
ENV SWAGGER_JSON=/docs/openapi.yaml