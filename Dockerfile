FROM alpine:3.24 AS core

COPY target/aarch64-unknown-linux-musl/release/backend .

COPY target/aarch64-unknown-linux-musl/release/db_migrate .

COPY target/dist dist

COPY backend/migrations migrations

RUN echo "#!/bin/sh" > entrypoint.sh
RUN echo "/db_migrate && /backend" >> /entrypoint.sh
RUN chmod +x /entrypoint.sh

EXPOSE 3000

ENTRYPOINT [ "/entrypoint.sh" ]
