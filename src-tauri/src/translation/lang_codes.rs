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
//! Language code conversion between ISO 639-3 (used app-wide, e.g. `eng`)
//! and the codes expected by the Google Translate API (mostly ISO 639-1
//! with quirks such as `zh-CN`, e.g. `eng` -> `en`).
//!
//! Single source of truth: [`ISO639_3_TO_GOOGLE`]. Both directions are
//! derived from it, so they cannot drift apart. Covers exactly the languages
//! listed in the frontend (`src/app/consts/languages.ts`).
//!
//! Note: `'auto'` is intentionally NOT part of the table. It is a
//! Google-specific source-language selector and is handled by the caller.

/// (ISO 639-3, Google) pairs. ISO keys are lowercase; Google values keep
/// the exact casing the API expects (`zh-CN`).
const ISO639_3_TO_GOOGLE: &[(&str, &str)] = &[
    ("afr", "af"),
    ("aka", "ak"),
    ("amh", "am"),
    ("ara", "ar"),
    ("aze", "az"),
    ("bel", "be"),
    ("ben", "bn"),
    ("bul", "bg"),
    ("cat", "ca"),
    ("ces", "cs"),
    ("cmn", "zh-CN"),
    ("cym", "cy"),
    ("dan", "da"),
    ("deu", "de"),
    ("ell", "el"),
    ("eng", "en"),
    ("epo", "eo"),
    ("est", "et"),
    ("fin", "fi"),
    ("fra", "fr"),
    ("guj", "gu"),
    ("heb", "he"),
    ("hin", "hi"),
    ("hrv", "hr"),
    ("hun", "hu"),
    ("hye", "hy"),
    ("ind", "id"),
    ("ita", "it"),
    ("jav", "jv"),
    ("jpn", "ja"),
    ("kan", "kn"),
    ("kat", "ka"),
    ("khm", "km"),
    ("kor", "ko"),
    ("lat", "la"),
    ("lav", "lv"),
    ("lit", "lt"),
    ("mal", "ml"),
    ("mar", "mr"),
    ("mkd", "mk"),
    ("mya", "my"),
    ("nep", "ne"),
    ("nld", "nl"),
    ("nob", "nb"),
    ("ori", "or"),
    ("pan", "pa"),
    ("pes", "fa"),
    ("pol", "pl"),
    ("por", "pt"),
    ("ron", "ro"),
    ("rus", "ru"),
    ("sin", "si"),
    ("slk", "sk"),
    ("slv", "sl"),
    ("sna", "sn"),
    ("spa", "es"),
    ("srp", "sr"),
    ("swe", "sv"),
    ("tam", "ta"),
    ("tel", "te"),
    ("tgl", "tl"),
    ("tha", "th"),
    ("tuk", "tk"),
    ("tur", "tr"),
    ("ukr", "uk"),
    ("urd", "ur"),
    ("uzb", "uz"),
    ("vie", "vi"),
    ("yid", "yi"),
    ("zul", "zu"),
];

/// Convert an app-wide ISO 639-3 code to the Google Translate API code.
///
/// Returns `None` for unknown codes (including `'auto'`); callers turn that
/// into a hard error instead of sending garbage to Google.
pub fn to_google_code(iso639_3: &str) -> Option<&'static str> {
    let needle = iso639_3.trim().to_lowercase();
    ISO639_3_TO_GOOGLE
        .iter()
        .find(|(iso, _)| *iso == needle)
        .map(|(_, google)| *google)
}

/// Convert a Google Translate API language code back to ISO 639-3.
///
/// Returns `None` for unknown codes, so response types never leak
/// non-639-3 codes into the rest of the app.
pub fn from_google_code(google_code: &str) -> Option<&'static str> {
    let needle = google_code.trim();
    ISO639_3_TO_GOOGLE
        .iter()
        .find(|(_, google)| google.eq_ignore_ascii_case(needle))
        .map(|(iso, _)| *iso)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn forward_common_languages() {
        assert_eq!(to_google_code("eng"), Some("en"));
        assert_eq!(to_google_code("rus"), Some("ru"));
        assert_eq!(to_google_code("spa"), Some("es"));
        assert_eq!(to_google_code("deu"), Some("de"));
        assert_eq!(to_google_code("fra"), Some("fr"));
    }

    #[test]
    fn forward_google_specific_quirks() {
        // Mandarin: Simplified Chinese only (no Traditional variant in the app).
        assert_eq!(to_google_code("cmn"), Some("zh-CN"));
        assert_eq!(to_google_code("heb"), Some("he"));
        assert_eq!(to_google_code("jpn"), Some("ja"));
        assert_eq!(to_google_code("jav"), Some("jv"));
        assert_eq!(to_google_code("nob"), Some("nb"));
        assert_eq!(to_google_code("pes"), Some("fa"));
        assert_eq!(to_google_code("tgl"), Some("tl"));
    }

    #[test]
    fn lookup_is_case_and_whitespace_insensitive() {
        assert_eq!(to_google_code(" ENG "), Some("en"));
        assert_eq!(to_google_code("Rus"), Some("ru"));
        assert_eq!(from_google_code(" EN "), Some("eng"));
        assert_eq!(from_google_code("ZH-cn"), Some("cmn"));
    }

    #[test]
    fn unknown_codes_return_none() {
        assert_eq!(to_google_code(""), None);
        assert_eq!(to_google_code("auto"), None);
        assert_eq!(to_google_code("xx"), None);
        assert_eq!(to_google_code("english"), None);
        assert_eq!(from_google_code(""), None);
        assert_eq!(from_google_code("auto"), None);
        assert_eq!(from_google_code("xx"), None);
    }

    #[test]
    fn every_entry_round_trips() {
        for (iso, google) in ISO639_3_TO_GOOGLE {
            assert_eq!(to_google_code(iso), Some(*google), "forward: {iso}");
            assert_eq!(from_google_code(google), Some(*iso), "reverse: {google}");
        }
    }

    #[test]
    fn table_has_no_duplicates() {
        let mut iso_codes = HashSet::new();
        let mut google_codes = HashSet::new();
        for (iso, google) in ISO639_3_TO_GOOGLE {
            assert!(iso_codes.insert(*iso), "duplicate ISO code: {iso}");
            assert!(
                google_codes.insert(google.to_lowercase()),
                "duplicate Google code: {google}"
            );
        }
    }
}
