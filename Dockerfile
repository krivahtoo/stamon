#################
# Build
#################

## Node

FROM node:18-alpine3.18 AS frontend

RUN npm install -g pnpm

WORKDIR /app
COPY ./frontend/package.json ./frontend/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY ./frontend .
RUN pnpm build

## Rust

# Start with a rust alpine image
FROM rust:1.78.0-alpine3.18 AS build

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# avoid checking the queries
ENV SQLX_OFFLINE=true

# install the build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# set the workdir
WORKDIR /app

# do a release build
# make sure to use cache to avoid unnecessary rebuild
# use a bind mount to avoid copying the whole source into the container
RUN --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/usr/local/cargo/git/ \
    cargo build --locked --release --package server && \
    cp ./target/release/server /bin/server


#################
# Runtime
################

# use a plain alpine image for runtime
# the alpine version needs to match the builder
FROM alpine:3.21

# set assets path
ENV ASSETS_PATH="/assets/"
# set data path
ENV DATA_PATH="/app/data"

# Create a volume for persistent data storage
VOLUME /app/data

# install the runtime dependencies
RUN apk add --no-cache libgcc

# copy the binary from the build stage
COPY --from=build /bin/server /bin/

# copy frontend assets
COPY --from=frontend /app/build /assets

# set the entrypoint
ENTRYPOINT ["/bin/server"]

# Document that the container listens on port 3000
# Note: This doesn't actually publish the port. Use -p flag with docker run or ports in docker-compose.yml
EXPOSE 3000
