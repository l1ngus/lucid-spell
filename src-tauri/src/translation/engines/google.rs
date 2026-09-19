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
use reqwest::header::{HeaderValue, ACCEPT, REFERER, USER_AGENT};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleTranslateError {
    #[error("invalid language")]
    InvalidLanguage,
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("http {status}: {body}")]
    Http {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("invalid response")]
    InvalidResponse,
}

pub async fn translate_google(
    client: &reqwest::Client,
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<TranslationResponse, GoogleTranslateError> {
    // Обработка пустого текста
    if text.trim().is_empty() {
        return Ok(TranslationResponse {
            translation: String::new(),
            detected_source_lang: None,
            source_correction: None,
        });
    }

    // Обработка исходного языка
    let source = source_lang.trim().to_lowercase();

    // Обработка целевого языка
    let target = target_lang.trim().to_lowercase();
    if target.is_empty() {
        return Err(GoogleTranslateError::InvalidLanguage);
    }

    // Выполняем запрос с заголовками Google ТОЛЬКО для этого запроса
    let response = client
        .get("https://translate.googleapis.com/translate_a/single")
        .header(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0",
            ),
        )
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        .header(
            REFERER,
            HeaderValue::from_static("https://translate.google.com"),
        )
        .query(&[
            ("client", "gtx"),
            ("sl", source.as_str()),
            ("tl", target.as_str()),
            ("dt", "t"),
            ("q", text),
        ])
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(GoogleTranslateError::Http {
            status,
            body: body.chars().take(300).collect(),
        });
    }

    // Парсим JSON ответ
    let json: Value =
        serde_json::from_str(&body).map_err(|_| GoogleTranslateError::InvalidResponse)?;

    // 1. Извлекаем переведенный текст
    let segments = json
        .get(0)
        .and_then(Value::as_array)
        .ok_or(GoogleTranslateError::InvalidResponse)?;

    let mut translated = String::with_capacity(text.len());
    for segment in segments {
        if let Some(piece) = segment.get(0).and_then(Value::as_str) {
            translated.push_str(piece);
        }
    }

    // 2. Извлекаем определенный исходный язык
    let detected_source_lang = json
        .get(2)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty() && s != "auto");

    // 3. Извлекаем исправление исходного текста
    let source_correction = json
        .get(7)
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .or_else(|| {
            json.get(8)
                .and_then(Value::as_array)
                .and_then(|arr| arr.first())
                .and_then(Value::as_str)
                .map(|s| s.to_string())
        })
        .filter(|s| !s.is_empty());

    Ok(TranslationResponse {
        translation: translated,
        detected_source_lang,
        source_correction,
    })
}
