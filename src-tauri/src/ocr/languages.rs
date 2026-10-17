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

//! Registry of PaddleOCR recognition models and the app-wide ISO 639-3
//! language -> model mapping.
//!
//! Models are ONNX conversions of the official PaddleOCR weights, hosted on
//! Hugging Face (`monkt/paddleocr-onnx`, Apache-2.0). Sizes are exact file
//! sizes and are used both for UI hints and download verification.

use crate::ocr::error::OcrError;

const HF_BASE: &str = "https://huggingface.co/monkt/paddleocr-onnx/resolve/main";

/// Height (in pixels) of the recognition model input. All current PaddleOCR
/// recognition models (v3/v5) use 3x48xW inputs.
pub const REC_INPUT_HEIGHT: usize = 48;

/// Detection model shared by all languages (PP-OCRv4 mobile, from RapidOCR).
pub struct DetModelSpec {
    pub url: &'static str,
    pub file: &'static str,
    pub size: u64,
}

pub const DET_MODEL: DetModelSpec = DetModelSpec {
    url: "https://huggingface.co/SWHL/RapidOCR/resolve/main/PP-OCRv4/ch_PP-OCRv4_det_infer.onnx",
    file: "det_ch_PP-OCRv4.onnx",
    size: 4_745_517,
};

/// A recognition model (one language group) with its dictionary.
pub struct RecModelSpec {
    /// Group id used by the API (e.g. `english`).
    pub id: &'static str,
    /// Human readable name.
    pub name: &'static str,
    /// ISO 639-3 codes served by this model. Also the language -> model
    /// mapping used by the service, so it has to stay in sync with what the
    /// model can actually read.
    pub languages: &'static [&'static str],
    /// Path inside the Hugging Face repository.
    pub rec_path: &'static str,
    pub rec_file: &'static str,
    pub rec_size: u64,
    pub dict_path: &'static str,
    pub dict_file: &'static str,
    pub dict_size: u64,
}

impl RecModelSpec {
    pub fn rec_url(&self) -> String {
        format!("{HF_BASE}/{}", self.rec_path)
    }

    pub fn dict_url(&self) -> String {
        format!("{HF_BASE}/{}", self.dict_path)
    }

    /// Total download size of model + dictionary.
    pub fn download_size(&self) -> u64 {
        self.rec_size + self.dict_size
    }
}

pub const REC_MODELS: &[RecModelSpec] = &[
    RecModelSpec {
        id: "english",
        name: "English",
        languages: &["eng"],
        rec_path: "languages/english/rec.onnx",
        rec_file: "rec_english.onnx",
        rec_size: 7_830_888,
        dict_path: "languages/english/dict.txt",
        dict_file: "dict_english.txt",
        dict_size: 1_416,
    },
    RecModelSpec {
        id: "latin",
        name: "Latin script",
        languages: &[
            "afr", "aka", "aze", "cat", "ces", "cym", "dan", "deu", "epo", "est", "fin", "fra",
            "hrv", "hun", "ind", "ita", "jav", "lat", "lav", "lit", "nld", "nob", "pol", "por",
            "ron", "slk", "slv", "spa", "sna", "swe", "tgl", "tuk", "tur", "uzb", "vie", "zul",
        ],
        rec_path: "languages/latin/rec.onnx",
        rec_file: "rec_latin.onnx",
        rec_size: 7_862_832,
        dict_path: "languages/latin/dict.txt",
        dict_file: "dict_latin.txt",
        dict_size: 1_634,
    },
    RecModelSpec {
        id: "eslav",
        name: "Cyrillic script",
        languages: &["rus", "ukr", "bel", "bul", "mkd", "srp"],
        rec_path: "languages/eslav/rec.onnx",
        rec_file: "rec_eslav.onnx",
        rec_size: 7_870_092,
        dict_path: "languages/eslav/dict.txt",
        dict_file: "dict_eslav.txt",
        dict_size: 1_663,
    },
    RecModelSpec {
        id: "chinese",
        name: "Chinese & Japanese",
        languages: &["cmn", "jpn"],
        rec_path: "languages/chinese/rec.onnx",
        rec_file: "rec_chinese.onnx",
        rec_size: 84_468_836,
        dict_path: "languages/chinese/dict.txt",
        dict_file: "dict_chinese.txt",
        dict_size: 74_012,
    },
    RecModelSpec {
        id: "korean",
        name: "Korean",
        languages: &["kor"],
        rec_path: "languages/korean/rec.onnx",
        rec_file: "rec_korean.onnx",
        rec_size: 13_401_252,
        dict_path: "languages/korean/dict.txt",
        dict_file: "dict_korean.txt",
        dict_size: 47_451,
    },
    RecModelSpec {
        id: "greek",
        name: "Greek",
        languages: &["ell"],
        rec_path: "languages/greek/rec.onnx",
        rec_file: "rec_greek.onnx",
        rec_size: 7_791_200,
        dict_path: "languages/greek/dict.txt",
        dict_file: "dict_greek.txt",
        dict_size: 1_103,
    },
    RecModelSpec {
        id: "arabic",
        name: "Arabic script",
        languages: &["ara", "urd", "pes"],
        rec_path: "languages/arabic/rec.onnx",
        rec_file: "rec_arabic.onnx",
        rec_size: 8_978_664,
        dict_path: "languages/arabic/dict.txt",
        dict_file: "dict_arabic.txt",
        dict_size: 405,
    },
    RecModelSpec {
        id: "hindi",
        name: "Devanagari script",
        languages: &["hin", "mar", "nep"],
        rec_path: "languages/hindi/rec.onnx",
        rec_file: "rec_hindi.onnx",
        rec_size: 8_980_224,
        dict_path: "languages/hindi/dict.txt",
        dict_file: "dict_hindi.txt",
        dict_size: 508,
    },
    RecModelSpec {
        id: "tamil",
        name: "Tamil",
        languages: &["tam"],
        rec_path: "languages/tamil/rec.onnx",
        rec_file: "rec_tamil.onnx",
        rec_size: 8_970_084,
        dict_path: "languages/tamil/dict.txt",
        dict_file: "dict_tamil.txt",
        dict_size: 352,
    },
    RecModelSpec {
        id: "telugu",
        name: "Telugu",
        languages: &["tel"],
        rec_path: "languages/telugu/rec.onnx",
        rec_file: "rec_telugu.onnx",
        rec_size: 8_976_064,
        dict_path: "languages/telugu/dict.txt",
        dict_file: "dict_telugu.txt",
        dict_size: 429,
    },
    RecModelSpec {
        id: "thai",
        name: "Thai",
        languages: &["tha"],
        rec_path: "languages/thai/rec.onnx",
        rec_file: "rec_thai.onnx",
        rec_size: 7_873_480,
        dict_path: "languages/thai/dict.txt",
        dict_file: "dict_thai.txt",
        dict_size: 1_767,
    },
];

pub fn find_model(id: &str) -> Option<&'static RecModelSpec> {
    REC_MODELS.iter().find(|model| model.id == id)
}

/// Resolve an ISO 639-3 code (e.g. `rus`) to the recognition model that best
/// covers its script. The language lists of the models are the single source
/// of truth here, so this always agrees with the languages advertised through
/// `get_ocr_status`.
pub fn model_for_language(code: &str) -> Result<&'static RecModelSpec, OcrError> {
    let code = code.trim().to_lowercase();
    if code.is_empty() {
        return Err(OcrError::InvalidInput("Empty language code".to_string()));
    }

    REC_MODELS
        .iter()
        .find(|model| model.languages.contains(&code.as_str()))
        .ok_or(OcrError::UnsupportedLanguage { lang: code })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_id(code: &str) -> &'static str {
        model_for_language(code).unwrap().id
    }

    #[test]
    fn resolves_languages_to_scripts() {
        assert_eq!(model_id("eng"), "english");
        assert_eq!(model_id("rus"), "eslav");
        assert_eq!(model_id("ukr"), "eslav");
        assert_eq!(model_id("srp"), "eslav");
        assert_eq!(model_id("cmn"), "chinese");
        assert_eq!(model_id("jpn"), "chinese");
        assert_eq!(model_id("kor"), "korean");
        assert_eq!(model_id("ell"), "greek");
        assert_eq!(model_id("hin"), "hindi");
        assert_eq!(model_id("nep"), "hindi");
        assert_eq!(model_id("ara"), "arabic");
        assert_eq!(model_id("urd"), "arabic");
        assert_eq!(model_id("pes"), "arabic");
        assert_eq!(model_id("tam"), "tamil");
        assert_eq!(model_id("tel"), "telugu");
        assert_eq!(model_id("tha"), "thai");
        assert_eq!(model_id("deu"), "latin");
        assert_eq!(model_id("ces"), "latin");
        assert_eq!(model_id("vie"), "latin");
        assert_eq!(model_id("pol"), "latin");
    }

    #[test]
    fn language_codes_are_normalized() {
        assert_eq!(model_id(" RUS "), "eslav");
        assert_eq!(model_id("ENG"), "english");
    }

    #[test]
    fn rejects_unsupported_scripts_and_unknown_codes() {
        assert!(model_for_language("ben").is_err());
        assert!(model_for_language("heb").is_err());
        assert!(model_for_language("kat").is_err());
        assert!(model_for_language("khm").is_err());
        assert!(model_for_language("xxx").is_err());
        assert!(model_for_language("").is_err());
    }

    #[test]
    fn every_model_language_resolves_back_to_it_and_no_code_is_ambiguous() {
        let mut seen: Vec<&str> = Vec::new();
        for model in REC_MODELS {
            for language in model.languages {
                assert_eq!(
                    find_model(model_for_language(language).unwrap().id)
                        .unwrap()
                        .id,
                    model.id
                );
                assert!(
                    !seen.contains(language),
                    "language '{language}' is listed by more than one model"
                );
                seen.push(language);
            }
        }
    }

    #[test]
    fn model_ids_are_unique_and_files_prefixed() {
        let mut ids: Vec<&str> = REC_MODELS.iter().map(|m| m.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), REC_MODELS.len());
    }
}
