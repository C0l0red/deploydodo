FROM alpine:3.24 AS core

COPY target/aarch64-unknown-linux-musl/release/backend .

COPY target/dist dist

COPY backend/migrations migrations

EXPOSE 3000

ENTRYPOINT [ "/backend" ]
