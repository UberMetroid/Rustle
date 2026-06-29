# Rustle - Wordle Clone in Rust & WebAssembly

<p align="center">
  <img src="frontend/public/favicon.png?v=0.1.26" alt="Rustle Logo" width="128" height="128">
</p>

Rustle is an optimized, responsive, and secure clone of the popular Wordle game built using Rust, Yew, and WebAssembly, served by a native Axum backend.

---

## Container Installation

### Option 1: Docker Compose (Recommended)

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  rustle:
    image: ubermetroid/rustle:0.1.26
    container_name: rustle
    restart: unless-stopped
    ports:
      - 4409:4409
    environment:
      PORT: 4409
      SITE_TITLE: Rustle
      RUSTLE_PIN: "" # Optional Access PIN
      BASE_URL: http://localhost:4409
      ALLOWED_ORIGINS: "*"
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4409`.

### Option 2: Docker CLI

Run the following command to start the container:

```bash
docker run -d \
  --name rustle \
  --restart unless-stopped \
  -p 4409:4409 \
  -e PORT=4409 \
  -e SITE_TITLE=Rustle \
  -e BASE_URL=http://localhost:4409 \
  -e ALLOWED_ORIGINS="*" \
  ubermetroid/rustle:0.1.26
```

---

## Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4409` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Rustle` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4409` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `RUSTLE_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header (true/false). | `false` |
| `ENABLE_THEMES` | Enable the Super Metroid theme selector in the navigation header (true/false). | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header (true/false). | `false` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before locking out the user client IP address. | `5` |

## Repository Structure

```
.
├── Cargo.toml
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── handlers.rs
│       ├── utils.rs
│       ├── login.html
│       └── auth/
│           ├── mod.rs
│           ├── lockout.rs
│           ├── crypto.rs
│           ├── handlers.rs
│           └── middleware.rs
└── frontend/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        ├── app_effects.rs
        ├── app_state.rs
        ├── constants.rs
        ├── index.css
        ├── tailwind.css
        ├── app/
        ├── components/
        ├── constants/
        ├── helpers/
        └── i18n/
```
