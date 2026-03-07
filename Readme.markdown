# QR Code Generator — Rezel

A full-stack web application for generating QR codes from text or URLs, with optional URL shortening. Built for [Rezel](https://rezel.net).

## Architecture

The project is composed of three services:

| Service | Technology | Port | Role |
|---------|-----------|------|------|
| **Backend** | Rust / Rocket | 8000 | HTTP API — generates QR codes, proxies URL shortening |
| **Frontend** | Astro / nginx | 80 | Single-page web UI |
| **URL Shortener** | Rust / Actix-web (`rs-short`) | 8080 | Shortens URLs before encoding |

## Setup

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)

### Quick start (recommended)

```bash
git clone <repo-url>
cd qrcode
docker-compose up --build
```

Then open `http://localhost` in your browser.

### Without Docker

**Backend** — requires Rust stable (`rustup`), `pkg-config`, `libssl-dev`:

```bash
cargo build --release
./target/release/qrcodegen
```

**Frontend** — requires Node.js 20+:

```bash
cd site
npm install
npm run dev      # development server with hot reload
# or
npm run build    # production static build
npm run preview  # preview the production build
```

**URL shortener** — see `rs-short/` and configure `rs-short/config.toml` (copy from `rs-short/config.toml.sample`).

## Configuration

### Backend environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RS_SHORT_URL` | `http://localhost:8080` | URL of the rs-short service |
| `ROCKET_ADDRESS` | `0.0.0.0` | Address for Rocket to bind |
| `ROCKET_PORT` | `8000` | Port for Rocket to bind |

### Frontend environment variables (build-time)

| Variable | Default | Description |
|----------|---------|-------------|
| `PUBLIC_API_URL` | `http://localhost:8000` | Backend API base URL, baked into the static build |

To set it at build time:

```bash
docker build --build-arg PUBLIC_API_URL=http://your-server:8000 -t qrcodegen-frontend ./site
```

### Docker Compose environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RS_INSTANCE_HOSTNAME` | `https://s.rezel.net` | Public hostname for shortened URLs |
| `PUBLIC_API_URL` | `http://localhost:8000` | Backend URL seen by browser clients |

### rs-short (`rs-short/config.toml`)

Copy the sample config and edit it:

```bash
cp rs-short/config.toml.sample rs-short/config.toml
```

Key settings:

| Key | Description |
|-----|-------------|
| `listening_address` | Bind address (default `0.0.0.0:8080`) |
| `database_path` | SQLite path or Postgres/MySQL connection string |
| `instance_hostname` | Public domain for shortened URLs |
| `cookie_key` | **Must be changed** — base64-encoded 64-byte secret key |
| `phishing_password` | Admin password for flagging phishing links |
| `captcha_difficulty` | `0` (easiest) to `5` (hardest) |
| `theme` | `light`, `dark`, or `custom` |

## API

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/qrcode/SVG/<content>/<level>` | Returns `{ "message": "<svg string>" }` |
| `GET` | `/qrcode/JPG/<content>/<level>` | Returns `{ "message": "<base64 jpg>" }` |
| `POST` | `/shorten` | Proxies to rs-short; body: `{ "url": "..." }`, returns `{ "short_url": "..." }` |

**Error correction levels:**

| Value | Level | Data recovery |
|-------|-------|--------------|
| `1` | Low | ~7% |
| `2` | Medium | ~15% |
| `3` | Quartile | ~25% |
| `4` | High | ~30% |

## Development

Run the demo CLI (no server needed):

```bash
cargo run --example qrcodegen-demo   # prints QR codes to terminal and generates an SVG sample
cargo run --example test1            # generates an SVG for "Hello, world!"
```

## License

The core QR code generation library (`src/lib.rs`) is based on [Project Nayuki's QR Code generator](https://www.nayuki.io/page/qr-code-generator-library), released under the MIT License.
