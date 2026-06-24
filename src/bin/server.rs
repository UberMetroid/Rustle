// Copyright (C) 2026 Jeryd
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

//! Native server backend using Axum.
//! Serves the static compiled frontend from the `/dist` directory.

#[cfg(not(target_arch = "wasm32"))]
mod native_server {
    use axum::{
        extract::State,
        http::{header, HeaderMap, StatusCode},
        middleware::{self, Next},
        response::{Html, IntoResponse, Response},
        routing::{get, post},
        Json, Router,
    };
    use serde::Deserialize;
    use serde_json::json;
    use std::net::SocketAddr;
    use std::path::Path;
    use tower_http::cors::CorsLayer;
    use tower_http::services::{ServeDir, ServeFile};

    #[derive(Clone)]
    pub struct AppState {
        pub pin: Option<String>,
        pub site_title: String,
        #[allow(dead_code)]
        pub allowed_origins: String,
    }

    #[derive(Deserialize)]
    pub struct VerifyPinPayload {
        pub pin: Option<String>,
    }

    // Embed premium Login HTML
    const LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{SITE_TITLE}</title>
    <style>
        :root {
            --bg-color: #0b0f19;
            --card-bg: rgba(17, 24, 39, 0.7);
            --card-border: rgba(255, 255, 255, 0.08);
            --primary: #4f46e5;
            --primary-hover: #4338ca;
            --text-main: #f3f4f6;
            --text-muted: #9ca3af;
            --error-color: #ef4444;
        }
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        body {
            font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background-color: var(--bg-color);
            color: var(--text-main);
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            overflow: hidden;
            position: relative;
        }
        body::before, body::after {
            content: '';
            position: absolute;
            width: 300px;
            height: 300px;
            border-radius: 50%;
            background: radial-gradient(circle, rgba(79, 70, 229, 0.15) 0%, rgba(0,0,0,0) 70%);
            z-index: 0;
        }
        body::before {
            top: 15%;
            left: 20%;
        }
        body::after {
            bottom: 15%;
            right: 20%;
        }
        .container {
            position: relative;
            z-index: 1;
            width: 100%;
            max-width: 400px;
            padding: 24px;
        }
        .card {
            background-color: var(--card-bg);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border: 1px solid var(--card-border);
            border-radius: 16px;
            padding: 32px;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.3), 0 8px 10px -6px rgba(0, 0, 0, 0.3);
            text-align: center;
            transition: transform 0.3s ease, box-shadow 0.3s ease;
        }
        .card:hover {
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.4), 0 10px 10px -5px rgba(0, 0, 0, 0.4);
        }
        .logo-container {
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 64px;
            height: 64px;
            border-radius: 12px;
            background: linear-gradient(135deg, #4f46e5 0%, #312e81 100%);
            margin-bottom: 24px;
            box-shadow: 0 4px 12px rgba(79, 70, 229, 0.3);
        }
        .logo-icon {
            font-size: 28px;
            font-weight: bold;
            color: #ffffff;
            letter-spacing: -1px;
        }
        h1 {
            font-size: 24px;
            font-weight: 700;
            margin-bottom: 8px;
            background: linear-gradient(to right, #ffffff, #9ca3af);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        p {
            font-size: 14px;
            color: var(--text-muted);
            margin-bottom: 24px;
        }
        .input-group {
            margin-bottom: 20px;
            text-align: left;
        }
        label {
            display: block;
            font-size: 12px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--text-muted);
            margin-bottom: 6px;
        }
        input {
            width: 100%;
            padding: 12px 16px;
            background-color: rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 8px;
            color: #ffffff;
            font-size: 16px;
            letter-spacing: 0.15em;
            text-align: center;
            outline: none;
            transition: border-color 0.2s ease, box-shadow 0.2s ease;
        }
        input:focus {
            border-color: var(--primary);
            box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.25);
        }
        button {
            width: 100%;
            padding: 12px;
            background-color: var(--primary);
            color: #ffffff;
            font-size: 16px;
            font-weight: 600;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            transition: background-color 0.2s ease, transform 0.1s ease;
            box-shadow: 0 4px 6px -1px rgba(79, 70, 229, 0.2);
        }
        button:hover {
            background-color: var(--primary-hover);
        }
        button:active {
            transform: scale(0.98);
        }
        .error-msg {
            font-size: 13px;
            color: var(--error-color);
            margin-top: 12px;
            min-height: 20px;
            opacity: 0;
            transition: opacity 0.2s ease;
        }
        .error-msg.visible {
            opacity: 1;
        }
        @keyframes shake {
            0%, 100% { transform: translateX(0); }
            25% { transform: translateX(-6px); }
            75% { transform: translateX(6px); }
        }
        .shake {
            animation: shake 0.2s ease-in-out 2;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="card" id="loginCard">
            <div class="logo-container">
                <span class="logo-icon">Ru</span>
            </div>
            <h1>{SITE_TITLE}</h1>
            <p>This application is protected by a PIN.</p>
            <form id="loginForm" onsubmit="handleLogin(event)">
                <div class="input-group">
                    <label for="pinInput">Enter PIN</label>
                    <input type="password" id="pinInput" maxlength="10" placeholder="••••" required autofocus autocomplete="current-password">
                </div>
                <button type="submit">Unlock App</button>
                <div class="error-msg" id="errorMsg">Invalid PIN. Please try again.</div>
            </form>
        </div>
    </div>

    <script>
        async function handleLogin(e) {
            e.preventDefault();
            const pinInput = document.getElementById('pinInput');
            const errorMsg = document.getElementById('errorMsg');
            const card = document.getElementById('loginCard');
            const pin = pinInput.value.trim();

            if (!pin) return;

            try {
                const response = await fetch('/api/verify-pin', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ pin })
                });

                const data = await response.json();

                if (response.ok && data.success) {
                    window.location.reload();
                } else {
                    showError();
                }
            } catch (err) {
                showError();
            }
        }

        function showError() {
            const pinInput = document.getElementById('pinInput');
            const errorMsg = document.getElementById('errorMsg');
            const card = document.getElementById('loginCard');
            
            errorMsg.classList.add('visible');
            card.classList.add('shake');
            pinInput.value = '';
            pinInput.focus();

            setTimeout(() => {
                card.classList.remove('shake');
            }, 400);
        }
    </script>
</body>
</html>"#;

    #[tokio::main]
    pub async fn run() {
        dotenvy::dotenv().ok();

        // 1. Ports
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(4409);

        // 2. Allowed origins
        let allowed_origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string());

        // 3. Site title
        let site_title = std::env::var("RUSTLE_TITLE")
            .or_else(|_| std::env::var("RUSTLE_SITE_TITLE"))
            .or_else(|_| std::env::var("SITE_TITLE"))
            .unwrap_or_else(|_| "Rustle".to_string());

        // 4. PIN
        let pin = std::env::var("RUSTLE_PIN")
            .or_else(|_| std::env::var("PIN"))
            .ok()
            .filter(|p| {
                !p.is_empty()
                    && p.chars().all(|c| c.is_ascii_digit())
                    && p.len() >= 4
                    && p.len() <= 10
            });

        let app_state = AppState {
            pin,
            site_title,
            allowed_origins: allowed_origins.clone(),
        };

        let cors = get_cors_layer(&allowed_origins);

        // Define main app router
        let api_routes = Router::new()
            .route("/pin-required", get(pin_required))
            .route("/verify-pin", post(verify_pin))
            .route("/auth-check", get(auth_check))
            .route("/logout", post(logout));

        let app = Router::new()
            .nest("/api", api_routes)
            .route("/asset-manifest.json", get(serve_asset_manifest))
            .route("/service-worker.js", get(serve_service_worker))
            .route("/", get(serve_index))
            .route("/index.html", get(serve_index))
            .fallback_service(ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
            .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware))
            .layer(cors)
            .with_state(app_state.clone());

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        println!("Server running natively on http://localhost:{}", port);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    // Handlers
    async fn pin_required(State(state): State<AppState>) -> impl IntoResponse {
        Json(json!({
            "required": state.pin.is_some(),
            "length": state.pin.as_ref().map(|p| p.len()).unwrap_or(0),
        }))
    }

    async fn verify_pin(
        State(state): State<AppState>,
        Json(payload): Json<VerifyPinPayload>,
    ) -> impl IntoResponse {
        let Some(ref config_pin) = state.pin else {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::SET_COOKIE,
                header::HeaderValue::from_static(
                    "RUSTLE_PIN=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0",
                ),
            );
            return (
                StatusCode::OK,
                headers,
                Json(json!({ "success": true })),
            )
                .into_response();
        };

        let pin_str = payload.pin.as_deref().unwrap_or("").trim();
        if pin_str.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "PIN is required." })),
            )
                .into_response();
        }

        if safe_compare(pin_str, config_pin) {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::SET_COOKIE,
                header::HeaderValue::from_str(&format!(
                    "RUSTLE_PIN={}; Path=/; HttpOnly; SameSite=Lax",
                    pin_str
                ))
                .unwrap(),
            );
            (
                StatusCode::OK,
                headers,
                Json(json!({ "success": true })),
            )
                .into_response()
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "success": false, "error": "Invalid PIN" })),
            )
                .into_response()
        }
    }

    async fn auth_check(
        headers: HeaderMap,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        if let Some(ref pin) = state.pin {
            if !is_authorized(&headers, pin) {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
        StatusCode::OK.into_response()
    }

    async fn logout() -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            header::HeaderValue::from_static(
                "RUSTLE_PIN=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0",
            ),
        );
        (
            StatusCode::OK,
            headers,
            Json(json!({ "success": true })),
        )
            .into_response()
    }

    #[derive(Deserialize)]
    struct IndexParams {
        d: Option<String>,
    }

    async fn serve_index(
        headers: HeaderMap,
        axum::extract::Query(params): axum::extract::Query<IndexParams>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        // If PIN is configured and user is NOT authenticated, serve the login page
        if let Some(ref pin) = state.pin {
            if !is_authorized(&headers, pin) {
                let login_rendered = LOGIN_HTML.replace("{SITE_TITLE}", &state.site_title);
                return Html(login_rendered).into_response();
            }
        }

        // Calculate current game date and index for Open Graph tags
        let game_date = if let Some(ref date_str) = params.d {
            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Local::now().date_naive())
        } else {
            chrono::Local::now().date_naive()
        };

        let first_game_date = chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap_or_default();
        let duration = game_date.signed_duration_since(first_game_date);
        let puzzle_index = duration.num_days() as i32;

        let is_today = game_date == chrono::Local::now().date_naive();
        let display_title = if is_today {
            format!("{} #{}", state.site_title, puzzle_index)
        } else {
            format!("{} #{} (Archive - {})", state.site_title, puzzle_index, game_date.format("%Y-%m-%d"))
        };

        let display_description = format!(
            "Play {} #{}: Can you guess the secret 5-letter word in 6 attempts? Solve the daily puzzle using Metroid themes, dark mode, and responsive layouts.",
            state.site_title, puzzle_index
        );

        // Serve the normal index.html with title and metadata replaced
        let path = std::path::Path::new("dist/index.html");
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                let mut rendered = if let Some(start_pos) = content.find("<title>") {
                    if let Some(end_pos) = content[start_pos..].find("</title>") {
                        let actual_end = start_pos + end_pos;
                        let mut new_content = content[..start_pos + 7].to_string();
                        new_content.push_str(&display_title);
                        new_content.push_str(&content[actual_end..]);
                        new_content
                    } else {
                        content.replace("<title>Rustle</title>", &format!("<title>{}</title>", display_title))
                    }
                } else {
                    content.replace("<title>Rustle</title>", &format!("<title>{}</title>", display_title))
                };

                // Replace main description
                rendered = rendered.replace(
                    "content=\"Rustle - Wordle clone in pure Rust\"",
                    &format!("content=\"{}\"", display_description),
                );

                // Replace Open Graph and Twitter placeholders
                rendered = rendered
                    .replace("content=\"Rustle (Title)\"", &format!("content=\"{}\"", display_title))
                    .replace("content=\"Rustle (Description)\"", &format!("content=\"{}\"", display_description));

                Html(rendered).into_response()
            }
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }

    async fn serve_service_worker() -> impl IntoResponse {
        serve_static_file("dist/public/service-worker.js", "application/javascript").await
    }

    async fn serve_asset_manifest() -> impl IntoResponse {
        let manifest = build_asset_manifest();
        Json(manifest)
    }

    // Middleware & helpers
    async fn auth_middleware(
        State(state): State<AppState>,
        headers: HeaderMap,
        request: axum::extract::Request,
        next: Next,
    ) -> Response {
        let Some(ref pin) = state.pin else {
            return next.run(request).await;
        };

        let path = request.uri().path();
        if path == "/api/pin-required" || path == "/api/verify-pin" {
            return next.run(request).await;
        }

        if is_authorized(&headers, pin) {
            return next.run(request).await;
        }

        // If they request the root page or an HTML file, let serve_index handle it (it will show the login HTML)
        if path == "/" || path == "/index.html" || path.ends_with(".html") {
            return next.run(request).await;
        }

        StatusCode::UNAUTHORIZED.into_response()
    }

    fn is_authorized(headers: &HeaderMap, pin: &str) -> bool {
        let cookie_pin = headers
            .get(header::COOKIE)
            .and_then(|c| c.to_str().ok())
            .and_then(|c_str| {
                c_str
                    .split(';')
                    .find(|s| s.trim().starts_with("RUSTLE_PIN="))
                    .and_then(|s| s.split('=').nth(1))
                    .map(|s| s.trim().to_string())
            });
        let header_pin = headers
            .get("x-pin")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        let provided_pin = cookie_pin.or(header_pin);

        match provided_pin {
            Some(prov) => safe_compare(&prov, pin),
            None => false,
        }
    }

    fn safe_compare(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut result = 0;
        for (x, y) in a.bytes().zip(b.bytes()) {
            result |= x ^ y;
        }
        result == 0
    }

    fn get_files_recursive(dir: &Path, base: &Path) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(get_files_recursive(&path, base));
                } else if let Ok(rel) = path.strip_prefix(base) {
                    if let Some(s) = rel.to_str() {
                        let url = format!("/{}", s.replace('\\', "/"));
                        files.push(url);
                    }
                }
            }
        }
        files
    }

    fn build_asset_manifest() -> Vec<String> {
        let dist_path = Path::new("dist");
        let mut files = get_files_recursive(dist_path, dist_path);
        if !files.contains(&"/favicon.png".to_string()) {
            files.push("/favicon.png".to_string());
        }
        if !files.contains(&"/public/favicon.png".to_string()) {
            files.push("/public/favicon.png".to_string());
        }
        if !files.contains(&"/public/manifest.json".to_string()) {
            files.push("/public/manifest.json".to_string());
        }
        files
    }

    fn get_cors_layer(allowed_origins_env: &str) -> CorsLayer {
        use axum::http::HeaderValue;
        use tower_http::cors::Any;

        if allowed_origins_env == "*" {
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        } else {
            let mut origins = Vec::new();
            for origin in allowed_origins_env.split(',') {
                let o = origin.trim();
                if !o.is_empty() {
                    if let Ok(val) = HeaderValue::from_str(o) {
                        origins.push(val);
                    }
                }
            }
            CorsLayer::new()
                .allow_origin(origins)
                .allow_methods(Any)
                .allow_headers(Any)
        }
    }

    async fn serve_static_file(path: &str, content_type: &str) -> Response {
        match tokio::fs::read(path).await {
            Ok(bytes) => ([(header::CONTENT_TYPE, content_type)], bytes).into_response(),
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    native_server::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Dummy main for wasm32-unknown-unknown target
}
