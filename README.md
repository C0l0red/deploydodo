# Deploydodo

Deploydodo is a self-hosted deployment dashboard: a Rust (Axum) backend, a React frontend, and an SSH-based terminal (`dodosh`) for connecting to servers — including the machine deploydodo itself runs on ("local" servers), which are just an SSH connection to a fixed host under the hood.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [cargo-make](https://github.com/sagiegurari/cargo-make) — `cargo install cargo-make`
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) — `cargo install sqlx-cli --no-default-features --features postgres,rustls`
- [Node.js](https://nodejs.org/) `v22.12.0` and npm
- [Docker](https://www.docker.com/) with Docker Compose (used to run Postgres locally, and to build/run the staging deployment)
- An SSH server running locally (e.g. macOS's built-in Remote Login) — deploydodo's "local server" feature connects to your own machine over SSH

## First-time development setup

### 1. Clone and install dependencies

```sh
git clone https://github.com/LibertyRepublic/deploydodo deploydodo
cd deploydodo
npm install --prefix frontend
```

### 2. Generate the local SSH keypair

deploydodo talks to the machine it's running on the same way it talks to any remote server: over SSH. `configure_rsa.sh` generates a keypair and authorizes it for the current user:

```sh
./configure_rsa.sh
```

This creates `key_rsa` / `key_rsa.pub` in the repo root and appends the public key to `~/.ssh/authorized_keys`. Both `key_rsa*` files are gitignored — treat `key_rsa` as a secret.

Make sure Remote Login (Mac OS) / an SSH daemon is actually enabled on your machine and reachable on the port you'll configure below, or the local-server terminal feature won't be able to connect.

### 3. Configure environment variables

Create a `.env` file in the repo root (gitignored) with:

```sh
DATABASE_URL=postgres://deploydodo:deploydodo@localhost:5432/deploydodo
LOCAL_SSH_HOSTNAME=localhost
LOCAL_SSH_PORT=22
LOCAL_SSH_USERNAME=<your local username>
LOCAL_SSH_PRIVATE_KEY=key_rsa
```

- `DATABASE_URL` must match the Postgres credentials in [`dev.docker-compose.yaml`](dev.docker-compose.yaml) (`deploydodo` / `deploydodo` / db `deploydodo` on port `5432` by default).
- `LOCAL_SSH_PRIVATE_KEY` is a **path** to the private key file (not the key contents) — point it at `key_rsa` from step 2.
- These are loaded once at startup by `backend/src/env.rs`; the process panics immediately if any are missing or invalid, so this is the one file to get right before running anything.

### 4. Start Postgres, run migrations, and start the app

```sh
cargo make dev
```

This single command:
1. Starts a Postgres container via `dev.docker-compose.yaml` (port `5432`, persisted in a named volume)
2. Runs the backend (`cargo run -p backend`), which applies pending `sqlx` migrations automatically on startup
3. Runs the frontend dev server (`npm run dev`) in parallel

The backend listens on `http://localhost:3000` and serves the API; the frontend dev server prints its own URL (typically `http://localhost:5173`) with API requests proxied through to the backend.

### 5. Complete first-run setup in the browser

Open the frontend URL. Since no admin user exists yet, you'll land on the setup wizard:
1. **Create an admin account** — name, email, password (8+ characters).
2. **Add a server** — choose "local" to connect to the machine deploydodo is running on (via the SSH config from step 3), or "remote" to add another server by hostname + SSH key.

From there you can open a terminal to any configured server from the dashboard.

## Everyday development

| Task | Command |
|---|---|
| Run backend + frontend + dev DB | `cargo make dev` |
| Run just the backend | `cargo make backend` |
| Run just the frontend | `cargo make frontend` |
| Add a new migration | `cargo make migration <name>` |
| Revert the last migration | `cargo make undo-migration` |
| Regenerate the frontend's typed API client from the backend's OpenAPI schema | `cargo make generate-schema` |

Migrations live in [`backend/migrations`](backend/migrations) and run automatically whenever the backend connects to the database — no separate migrate step needed in normal development.

## Building for staging/production

The Docker image expects pre-built artifacts rather than building Rust/Node inside the container, so build first, then bring up the stack:

```sh
cargo make build-backend-linux   # cross-compiles the backend for aarch64-unknown-linux-musl
cargo make build-frontend        # builds the frontend into target/dist
cargo make compose               # builds the Docker image and runs staging.docker-compose.yaml
```

`staging.docker-compose.yaml` additionally expects:
- A `local.env` file in the repo root — gitignored, not committed:
```sh
DATABASE_URL=postgres://deploydodo:deploydodo@dododb:5432/deploydodo
LOCAL_SSH_HOSTNAME=deploydodo.host
LOCAL_SSH_PORT=22
LOCAL_SSH_USERNAME=benjamin
LOCAL_SSH_PRIVATE_KEY=key_rsa
```
- The `key_rsa` private key generated in step 2, mounted into the container at `/key_rsa`.

The staging compose file runs Postgres and the app on an internal network, exposing only the app on port `3000`.

## Project layout

- [`backend/`](backend) — Axum API server, Postgres access via `sqlx`, OpenAPI schema generation
- [`dodosh/`](dodosh) — SSH/terminal crate used by the backend to connect to local and remote servers
- [`frontend/`](frontend) — React + Vite dashboard UI
- [`dev.docker-compose.yaml`](dev.docker-compose.yaml) / [`staging.docker-compose.yaml`](staging.docker-compose.yaml) — local and staging deployment stacks
- [`Makefile.toml`](Makefile.toml) — all `cargo make` task definitions
