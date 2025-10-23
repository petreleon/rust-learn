#############################################
# Build MinIO from source (no binary downloads)
# - Build stage: golang:1.24-alpine
# - Final stage: clean Alpine Linux
#############################################

FROM golang:1.24-alpine AS build

ARG TARGETARCH

ENV GOPATH=/go \
    CGO_ENABLED=0 \
    GOOS=linux

WORKDIR /src/minio

# Copy MinIO source from the workspace (submodule: third_party/minio)
COPY third_party/minio/ .

# Build the minio server from source
# NOTE: This will fetch Go modules from the network unless vendored.
RUN set -eux; \
    GOARCH="${TARGETARCH:-amd64}"; \
    echo "Building MinIO for GOARCH=${GOARCH}"; \
    GOARCH="$GOARCH" go build -trimpath -ldflags "-s -w" -o /out/minio .; \
    /out/minio --version || true

# --- Final runtime image ---
FROM alpine:3.20

# Minimal runtime deps
RUN apk add -U --no-cache ca-certificates

COPY --from=build /out/minio /usr/bin/minio

# Copy the upstream entrypoint script and make it executable
COPY third_party/minio/dockerscripts/docker-entrypoint.sh /usr/bin/docker-entrypoint.sh
RUN chmod +x /usr/bin/docker-entrypoint.sh

EXPOSE 9000 9001
VOLUME ["/data"]

# Use upstream entrypoint; default CMD is just "minio" (pass server args at runtime)
ENTRYPOINT ["/usr/bin/docker-entrypoint.sh"]
CMD ["minio"]
