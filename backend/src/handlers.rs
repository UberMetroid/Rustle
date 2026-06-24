// Copyright (C) 2026 UberMetroid
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

use crate::auth::{is_authorized, AppState, LOGIN_HTML};
use crate::utils::{build_asset_manifest, get_holiday_for_date};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexParams {
    pub d: Option<String>,
}

pub async fn serve_index(
    headers: HeaderMap,
    Query(params): Query<IndexParams>,
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

    let first_game_date = chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap_or_default();
    let duration = game_date.signed_duration_since(first_game_date);
    let puzzle_index = duration.num_days() as i32;

    let puzzle_num_str = if puzzle_index < 0 {
        format!("Beta #{}", puzzle_index.abs())
    } else {
        format!("#{}", puzzle_index + 1)
    };

    let holiday_info = get_holiday_for_date(game_date);

    let is_today = game_date == chrono::Local::now().date_naive();
    let display_title = if let Some((_, holiday_name)) = holiday_info {
        if is_today {
            format!(
                "{} {} - Special {} Edition!",
                state.site_title, puzzle_num_str, holiday_name
            )
        } else {
            format!(
                "{} {} ({} Archive)",
                state.site_title, puzzle_num_str, holiday_name
            )
        }
    } else if is_today {
        format!("{} {}", state.site_title, puzzle_num_str)
    } else {
        format!(
            "{} {} (Archive - {})",
            state.site_title,
            puzzle_num_str,
            game_date.format("%Y-%m-%d")
        )
    };

    let display_description = if let Some((_, holiday_name)) = holiday_info {
        format!(
            "Play {} {}: Celebrate {} with this special edition 5-letter word puzzle! Solve it using custom holiday theme colors.",
            state.site_title, puzzle_num_str, holiday_name
        )
    } else {
        format!(
            "Play {} {}: Can you guess the secret 5-letter word in 6 attempts? Solve the daily puzzle using Metroid themes, dark mode, and responsive layouts.",
            state.site_title, puzzle_num_str
        )
    };

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
                    content.replace(
                        "<title>Rustle</title>",
                        &format!("<title>{}</title>", display_title),
                    )
                }
            } else {
                content.replace(
                    "<title>Rustle</title>",
                    &format!("<title>{}</title>", display_title),
                )
            };

            // Replace main description
            rendered = rendered.replace(
                "content=\"Rustle - Wordle clone in pure Rust\"",
                &format!("content=\"{}\"", display_description),
            );

            // Replace Open Graph and Twitter placeholders
            rendered = rendered
                .replace(
                    "content=\"Rustle (Title)\"",
                    &format!("content=\"{}\"", display_title),
                )
                .replace(
                    "content=\"Rustle (Description)\"",
                    &format!("content=\"{}\"", display_description),
                );

            Html(rendered).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn serve_service_worker() -> impl IntoResponse {
    serve_static_file("dist/public/service-worker.js", "application/javascript").await
}

pub async fn serve_asset_manifest() -> impl IntoResponse {
    let manifest = build_asset_manifest();
    Json(manifest)
}

pub async fn serve_static_file(path: &str, content_type: &str) -> Response {
    match tokio::fs::read(path).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, content_type)], bytes).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
