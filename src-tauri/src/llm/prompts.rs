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
use minijinja::{context, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationPromptOutput {
    pub translation: String,
    pub source_correction: String,
}

const TRANSLATION_PROMPT_TEMPLATE: &str = r#"You are an expert linguist and professional translator. Your task is to translate the provided text from {{ source_lang }} to {{ target_lang }}, and verify the grammatical correctness of the source text.

Strictly adhere to the following rules:
1. Translate the text accurately, preserving the original meaning, tone, nuances, and formatting.
2. Analyze the source text for any spelling, grammar, or typographical errors. If the source text contains errors, provide the corrected version in the "sourceCorrection" field. If the source text is written correctly, leave the "sourceCorrection" field completely empty ("").
3. Output strictly and ONLY in valid JSON format.
4. Do not include any markdown formatting (e.g., ```json), conversational filler, explanations, or greetings.
5. Ensure any quotation marks or special characters inside the text fields are properly escaped to maintain valid JSON.

Expected JSON Output Format:
  {
    "translation": "<translated text goes here>",
    "sourceCorrection": "<corrected source text if errors exist, otherwise empty string>"
  }

Input Data:
Source Language: {{ source_lang }}
Target Language: {{ target_lang }}
Text to translate: {{ text }}"#;

pub fn translation_prompt(
    text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<String, minijinja::Error> {
    let env = Environment::new();

    env.render_str(
        TRANSLATION_PROMPT_TEMPLATE,
        context! {
            text => text,
            source_lang => source_lang,
            target_lang => target_lang,
        },
    )
}

const OTHER_TRANSLATIONS_PROMPT_TEMPLATE: &str = r#"You are a professional translator focusing on natural, everyday language. Your task is to provide a few practical and common alternative translations of the source text.

Input data:
- sourceText: {{ source_text }}
- translatedText: {{ translated_text }} (Do NOT include this exact text in your output)
- sourceLang: {{ source_lang }}
- targetLang: {{ target_lang }}

Rules:
1. Analyze sourceText and its primary translatedText.
2. Provide ONLY common, widely-used, and natural alternative translations. Do NOT invent rare, archaic, overly creative, or highly specific dictionary edge-cases. Be pragmatic and modest.
3. Limit your output to a maximum of 3 to 5 best alternatives.
4. If sourceText is a SINGLE WORD (no spaces), return word alternatives using parts of speech:
{"otherTranslations":[{"part":"<part of speech>","translation":"<alternative translation>"}, ...]}
5. If sourceText contains spaces (phrase or sentence), return phrase/sentence alternatives without parts of speech:
{"otherTranslations":["<alternative translation 1>","<alternative translation 2>", ...]}
6. If there are no genuinely common or practical alternative translations (e.g., proper nouns, simple unambiguous terms, technical terms, numbers), return:
{"otherTranslations":[]}
7. Return only alternative translations in targetLang.
8. Do not add explanations, notes, markdown, or any text outside the JSON.
9. Return STRICTLY valid JSON."#;

pub fn other_translations_prompt(
    source_text: &str,
    translated_text: &str,
    source_lang: &str,
    target_lang: &str,
) -> Result<String, minijinja::Error> {
    let env = Environment::new();

    env.render_str(
        OTHER_TRANSLATIONS_PROMPT_TEMPLATE,
        context! {
            source_text => source_text,
            translated_text => translated_text,
            source_lang => source_lang,
            target_lang => target_lang,
        },
    )
}
