// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//

pub mod auth;
pub mod handlers;
pub mod utils;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

use auth::{auth_check, auth_middleware, logout, pin_required, verify_pin, AppState, security_headers_middleware};
use handlers::{serve_asset_manifest, serve_index, serve_service_worker};

#[tokio::main]
async fn main() {
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
            !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) && p.len() >= 4 && p.len() <= 10
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
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .layer(cors)
        .layer(middleware::from_fn(security_headers_middleware))
        .with_state(app_state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running natively on http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_cors_layer(allowed_origins_env: &str) -> tower_http::cors::CorsLayer {
    use axum::http::HeaderValue;
    use tower_http::cors::Any;

    if allowed_origins_env == "*" {
        tower_http::cors::CorsLayer::new()
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
        tower_http::cors::CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
