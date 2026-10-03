/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::models::TranslationResponse;
use crate::translation::lang_codes::{from_google_code, to_google_code};
use reqwest::StatusCode;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleTranslateError {
    #[error("invalid language")]
    InvalidLanguage,
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("http {status}: {body}")]
    Http { status: StatusCode, body: String },
    #[error("invalid response")]
    InvalidResponse,
}

/// Ручное URL-кодирование.
/// Кодирует пробелы как %20, а не как + (это критично для Google API).
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

pub async fn translate_google(
    client: &reqwest::Client,
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<TranslationResponse, GoogleTranslateError> {
    if text.trim().is_empty() {
        return Ok(TranslationResponse {
            translation: String::new(),
            detected_source_lang: None,
            source_correction: None,
        });
    }

    let source = source_lang.trim().to_lowercase();
    let target = target_lang.trim().to_lowercase();

    // The app speaks ISO 639-3; Google speaks ISO 639-1 plus quirks.
    // 'auto' source detection passes through; anything else must map,
    // otherwise we fail fast instead of sending garbage to Google.
    let source_code = if source == "auto" {
        "auto".to_string()
    } else {
        to_google_code(&source)
            .ok_or(GoogleTranslateError::InvalidLanguage)?
            .to_string()
    };
    let target_code = to_google_code(&target).ok_or(GoogleTranslateError::InvalidLanguage)?;

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&dj=1&dt=t&sl={}&tl={}&q={}",
        urlencode(&source_code),
        urlencode(target_code),
        urlencode(text)
    );

    let response = client.get(&url).send().await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(GoogleTranslateError::Http {
            status,
            body: body.chars().take(300).collect(),
        });
    }

    let json: Value =
        serde_json::from_str(&body).map_err(|_| GoogleTranslateError::InvalidResponse)?;

    let sentences = json
        .get("sentences")
        .and_then(Value::as_array)
        .ok_or(GoogleTranslateError::InvalidResponse)?;

    let mut translated = String::with_capacity(text.len());
    for sentence in sentences {
        if let Some(trans) = sentence.get("trans").and_then(Value::as_str) {
            translated.push_str(trans);
        }
    }

    // Google reports detection in its own codes; map back to ISO 639-3
    // so the response never leaks non-639-3 codes into the app.
    // Unknown codes are dropped (None) rather than passed through raw.
    let detected_source_lang = json
        .get("src")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty() && *s != "auto")
        .and_then(from_google_code)
        .map(str::to_string);

    let source_correction = json
        .get("spell")
        .and_then(|spell: &Value| spell.get("spell_res"))
        .and_then(Value::as_str)
        .map(|s: &str| s.to_string())
        .or_else(|| {
            json.get("correction")
                .and_then(Value::as_str)
                .map(|s: &str| s.to_string())
        })
        .filter(|s: &String| !s.is_empty());

    Ok(TranslationResponse {
        translation: translated,
        detected_source_lang,
        source_correction,
    })
}
