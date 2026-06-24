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

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

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
pub const LOGIN_HTML: &str = include_str!("login.html");

pub async fn pin_required(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "required": state.pin.is_some(),
        "length": state.pin.as_ref().map(|p| p.len()).unwrap_or(0),
    }))
}

pub async fn verify_pin(
    State(state): State<AppState>,
    Json(payload): Json<VerifyPinPayload>,
) -> impl IntoResponse {
    let Some(ref config_pin) = state.pin else {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            header::HeaderValue::from_static(
                "RUSTLE_PIN=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0",
            ),
        );
        return (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response();
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
                "RUSTLE_PIN={}; Path=/; HttpOnly; SameSite=Strict",
                hash_pin(pin_str)
            ))
            .unwrap(),
        );
        (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "error": "Invalid PIN" })),
        )
            .into_response()
    }
}

pub async fn auth_check(headers: HeaderMap, State(state): State<AppState>) -> impl IntoResponse {
    if let Some(ref pin) = state.pin {
        if !is_authorized(&headers, pin) {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }
    StatusCode::OK.into_response()
}

pub async fn logout() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        header::HeaderValue::from_static("RUSTLE_PIN=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
    );
    (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response()
}

pub async fn auth_middleware(
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

    if path == "/" || path == "/index.html" || path.ends_with(".html") {
        return next.run(request).await;
    }

    StatusCode::UNAUTHORIZED.into_response()
}

pub fn is_authorized(headers: &HeaderMap, pin: &str) -> bool {
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

    match (cookie_pin, header_pin) {
        (Some(cookie), _) => safe_compare(&cookie, &hash_pin(pin)),
        (None, Some(hdr)) => safe_compare(&hdr, pin),
        (None, None) => false,
    }
}

pub fn safe_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

pub fn hash_pin(pin: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(pin.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub async fn security_headers_middleware(
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    
    headers.insert("X-Frame-Options", header::HeaderValue::from_static("DENY"));
    headers.insert("X-Content-Type-Options", header::HeaderValue::from_static("nosniff"));
    headers.insert("Referrer-Policy", header::HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert(
        "Content-Security-Policy", 
        header::HeaderValue::from_static(
            "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; img-src 'self' data: blob: https:; connect-src 'self' ws: wss: http: https:; font-src 'self'; manifest-src 'self';"
        )
    );
    
    response
}
