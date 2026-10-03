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
pub fn clean_llm_output(llm_message: impl Into<String>) -> String {
    let input = llm_message.into();
    let mut result = String::with_capacity(input.len());

    // Удаляем think-блоки.
    let mut remaining = input.as_str();

    while let Some(start) = find_tag_ci(remaining, "<think>") {
        // Сохраняем текст перед <think>
        result.push_str(&remaining[..start]);

        let after_start = &remaining[start + "<think>".len()..];

        // Если </think> существует — продолжаем после него.
        // Если нет — считаем, что модель начала "думать" до конца ответа.
        if let Some(end) = find_tag_ci(after_start, "</think>") {
            remaining = &after_start[end + "</think>".len()..];
        } else {
            remaining = "";
            break;
        }
    }

    // Добавляем остаток после последнего think-блока.
    result.push_str(remaining);

    let mut cleaned = result.trim().to_string();

    // Убираем markdown code fences.
    cleaned = strip_code_fences(&cleaned);

    // Финальная нормализация.
    cleaned
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

/// Case-insensitive поиск ASCII-тега.
fn find_tag_ci(haystack: &str, tag: &str) -> Option<usize> {
    haystack.char_indices().find_map(|(idx, _)| {
        haystack[idx..]
            .get(..tag.len())
            .filter(|part| part.eq_ignore_ascii_case(tag))
            .map(|_| idx)
    })
}

/// Убирает ```...``` вокруг ответа, но сохраняет содержимое.
///
/// Поддерживает:
/// ```
/// text
/// ```
///
/// и:
/// ```json
/// {"translation": "..."}
/// ```
fn strip_code_fences(input: &str) -> String {
    let trimmed = input.trim();

    if !trimmed.starts_with("```") {
        return trimmed.to_string();
    }

    // Убираем первые ```
    let mut content = &trimmed[3..];

    // Убираем язык после открывающего fence:
    // ```json
    // ```rust
    // ```text
    if let Some(newline) = content.find('\n') {
        let first_line = content[..newline].trim();

        // Если первая строка похожа на указание языка,
        // считаем её частью fence.
        if !first_line.is_empty()
            && first_line
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '+')
        {
            content = &content[newline + 1..];
        }
    }

    // Если есть закрывающий fence — убираем его.
    if let Some(end) = content.rfind("```") {
        content = &content[..end];
    }

    content.trim().to_string()
}
