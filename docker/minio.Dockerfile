#############################################
# Build MinIO from source via `go install`
# - Build stage: golang:1.24-alpine
# - Final stage: clean Alpine Linux
#############################################

FROM golang:1.24-alpine AS build

ARG TARGETARCH

ENV CGO_ENABLED=0 \
    GOOS=linux

RUN apk add -U --no-cache ca-certificates git

# Install MinIO from source per upstream docs
RUN set -eux; \
    GOARCH="${TARGETARCH:-amd64}"; \
    echo "Installing MinIO for GOARCH=${GOARCH}"; \
    GOOS=linux GOARCH="$GOARCH" go install github.com/minio/minio@latest; \
    test -x /go/bin/minio; \
    /go/bin/minio --version || true

# --- Final runtime image ---
FROM alpine:3.20

# Minimal runtime deps
RUN apk add -U --no-cache ca-certificates

COPY --from=build /go/bin/minio /usr/bin/minio

EXPOSE 9000 9001
VOLUME ["/data"]

# Run MinIO; docker-compose will supply the full command (e.g., `server /data ...`)
ENTRYPOINT ["/usr/bin/minio"]
