# build backend
FROM rust:1.89 AS base
RUN cargo install --locked cargo-chef

FROM base AS planner

WORKDIR /app

COPY ./backend ./backend
COPY ./backend/Cargo.toml Cargo.toml

# Creates the manifest for dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder_backend

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
# Necessary for `init_db`
COPY --from=planner /app/backend/migrations/ migrations/

# Necessary for `init_db`
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Caches the target directory for intermediate build artifacts.
# `cargo chef cook` downloads, builds, and caches all of the dependencies for future builds.
RUN --mount=type=cache,target=target,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY ./backend .

# The finished binary is placed in /usr/local/bin because it can't be conditionally copied from the runtime stage
RUN --mount=type=cache,target=target,sharing=locked \
    SQLX_OFFLINE=true cargo build --release && mv /app/target/release/dasharr /usr/local/bin/dasharr_backend;

# build frontend
FROM node:lts-slim AS builder_frontend

WORKDIR /home/node/app

COPY ./frontend .

RUN --mount=type=cache,target=/root/.npm \
    npm ci --verbose --no-audit
# This should be npm run build
RUN npx vite build

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev openssl curl pkg-config ca-certificates nginx postgresql tini gcc

# install sqlx-cli to handle database migrations
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install sqlx-cli

COPY ./backend/migrations/ /migrations

COPY ./docker/initdb.sh /
RUN chmod +x /initdb.sh

COPY --from=builder_backend /usr/local/bin/dasharr_backend /usr/local/bin

COPY --from=builder_frontend /home/node/app/dist/ /usr/share/nginx/html
COPY ./docker/nginx.conf /etc/nginx/nginx.conf

# frontend
EXPOSE 80
# backend
EXPOSE 8080

ENTRYPOINT ["/usr/bin/tini", "--"]

CMD ["sh", "-c", "\
    cd / && /initdb.sh && \
    cd /usr/local/bin/ && ./dasharr_backend & \
    nginx -g 'daemon off;' & \
    wait"]
