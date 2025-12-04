use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::error::ConfigError;

/// User-provided configuration for the normalization pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizationConfig {
    pub special_tokens: BTreeMap<char, String>,
    pub transliterations: BTreeMap<char, String>,
    pub lowercase: bool,
    pub lowercase_extension: bool,
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        Self {
            special_tokens: BTreeMap::from([
                ('/', "-or-".to_string()),
                ('&', "-and-".to_string()),
                ('@', "-at-".to_string()),
                ('%', "-percent-".to_string()),
            ]),
            transliterations: BTreeMap::from([
                ('á', "a".to_string()),
                ('à', "a".to_string()),
                ('â', "a".to_string()),
                ('ä', "a".to_string()),
                ('ã', "a".to_string()),
                ('å', "a".to_string()),
                ('é', "e".to_string()),
                ('è', "e".to_string()),
                ('ê', "e".to_string()),
                ('ë', "e".to_string()),
                ('í', "i".to_string()),
                ('ì', "i".to_string()),
                ('î', "i".to_string()),
                ('ï', "i".to_string()),
                ('ó', "o".to_string()),
                ('ò', "o".to_string()),
                ('ô', "o".to_string()),
                ('ö', "o".to_string()),
                ('õ', "o".to_string()),
                ('ø', "o".to_string()),
                ('ú', "u".to_string()),
                ('ù', "u".to_string()),
                ('û', "u".to_string()),
                ('ü', "u".to_string()),
                ('ñ', "n".to_string()),
                ('ç', "c".to_string()),
                ('æ', "ae".to_string()),
                ('œ', "oe".to_string()),
                ('ß', "ss".to_string()),
            ]),
            lowercase: true,
            lowercase_extension: true,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct NormalizationOptions {
    lowercase: Option<bool>,
    lowercase_extension: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct NormalizationFileConfig {
    #[serde(default)]
    special_tokens: BTreeMap<String, String>,
    #[serde(default)]
    transliterations: BTreeMap<String, String>,
    #[serde(default)]
    options: NormalizationOptions,
}

impl NormalizationConfig {
    pub fn from_toml_str(contents: &str) -> Result<Self, ConfigError> {
        Self::from_toml_str_with_path(contents, None)
    }

    pub fn from_toml_str_with_path(
        contents: &str,
        path: Option<&Path>,
    ) -> Result<Self, ConfigError> {
        let parsed: NormalizationFileConfig = toml::from_str(contents).map_err(|error| {
            path.map_or_else(
                || ConfigError::from_parse_error(error),
                |p| ConfigError::from_parse_error_with_path(p, error),
            )
        })?;
        Self::apply_overrides(parsed)
    }

    fn apply_overrides(overrides: NormalizationFileConfig) -> Result<Self, ConfigError> {
        let mut config = NormalizationConfig::default();

        for (key, value) in overrides.special_tokens {
            let ch = ConfigError::char_from_key("special_tokens", &key)?;
            config.special_tokens.insert(ch, value);
        }

        for (key, value) in overrides.transliterations {
            let ch = ConfigError::char_from_key("transliterations", &key)?;
            config.transliterations.insert(ch, value);
        }

        if let Some(lowercase) = overrides.options.lowercase {
            config.lowercase = lowercase;
        }

        if let Some(lowercase_extension) = overrides.options.lowercase_extension {
            config.lowercase_extension = lowercase_extension;
        }

        Ok(config)
    }
}

/// Normalize a filename according to the fnorm rules using the default configuration.
#[must_use]
pub fn normalize(filename: &str) -> String {
    normalize_with_config(filename, &NormalizationConfig::default())
}

/// Normalize a filename using a provided configuration.
#[must_use]
pub fn normalize_with_config(filename: &str, config: &NormalizationConfig) -> String {
    // Step 1: Empty input
    if filename.is_empty() {
        return String::new();
    }

    // Step 2: Extension detection
    let (base_name, extension) = split_extension(filename.trim());

    // Steps 3-10: Normalization pipeline in a single pass for the base name
    let base = normalize_base(base_name, config);

    // Step 11: Extension normalization
    let normalized_extension = if config.lowercase_extension {
        extension.to_lowercase()
    } else {
        extension.to_string()
    };

    // Step 12: Reassembly
    if normalized_extension.is_empty() {
        base
    } else {
        format!("{base}.{normalized_extension}")
    }
}

/// Split a filename into base name and extension
/// Returns (`base_name`, extension) where extension does not include the dot
fn split_extension(filename: &str) -> (&str, &str) {
    if let Some(dot_pos) = filename.rfind('.') {
        if dot_pos == 0 {
            // Hidden file like ".bashrc" - treat entire name as extension
            ("", &filename[1..])
        } else if dot_pos == filename.len() - 1 {
            // Ends with lone dot - no extension
            (&filename[..dot_pos], "")
        } else {
            (&filename[..dot_pos], &filename[dot_pos + 1..])
        }
    } else {
        // No dot found - no extension
        (filename, "")
    }
}

fn normalize_base(base_name: &str, config: &NormalizationConfig) -> String {
    let trimmed = base_name.trim().trim_matches('.');

    if trimmed.is_empty() {
        return String::new();
    }

    let mut processed = String::with_capacity(trimmed.len() * 2);

    for ch in trimmed.chars() {
        let lower_iter: Box<dyn Iterator<Item = char>> = if config.lowercase {
            Box::new(ch.to_lowercase())
        } else {
            Box::new(std::iter::once(ch))
        };

        for lower in lower_iter {
            if let Some(replacement) = config.special_tokens.get(&lower) {
                processed.push_str(replacement);
                continue;
            }

            if let Some(replacement) = config.transliterations.get(&lower) {
                processed.push_str(replacement);
                continue;
            }

            match lower {
                ' ' | '–' | '—' | '\u{2018}' | '\u{2019}' | '\u{201C}' | '\u{201D}' => {
                    processed.push('-');
                }
                _ => {
                    let is_allowed_letter = if config.lowercase {
                        lower.is_ascii_lowercase()
                    } else {
                        lower.is_ascii_alphabetic()
                    };

                    if is_allowed_letter
                        || lower.is_ascii_digit()
                        || lower == '-'
                        || lower == '_'
                        || lower == '.'
                    {
                        processed.push(lower);
                    } else {
                        processed.push('-');
                    }
                }
            }
        }
    }

    let cleaned = cleanup_hyphens(&processed);
    cleaned.trim_start_matches('-').to_string()
}

/// Collapse consecutive hyphens into single hyphens
fn cleanup_hyphens(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_hyphen = false;

    for ch in text.chars() {
        if ch == '-' {
            if !prev_was_hyphen {
                result.push(ch);
                prev_was_hyphen = true;
            }
        } else {
            result.push(ch);
            prev_was_hyphen = false;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_basic_normalization() {
        assert_eq!(normalize("My File.txt"), "my-file.txt");
        assert_eq!(normalize("Another-File.DOC"), "another-file.doc");
    }

    #[test]
    fn test_spaces_to_hyphens() {
        assert_eq!(normalize("File With Spaces.txt"), "file-with-spaces.txt");
    }

    #[test]
    fn test_examples_from_spec() {
        // Examples from Section 5 of the functional spec
        assert_eq!(normalize("My Document.PDF"), "my-document.pdf");
        assert_eq!(normalize("File & Video.mov"), "file-and-video.mov");
        assert_eq!(normalize("tcp/udp guide.md"), "tcp-or-udp-guide.md");
        assert_eq!(normalize("café menu.txt"), "cafe-menu.txt");
        assert_eq!(normalize("rock'n'roll.txt"), "rock-n-roll.txt");
        assert_eq!(normalize("Résumé"), "resume");
        assert_eq!(normalize(".Hidden File"), ".hidden file"); // Known limitation
    }

    #[test]
    fn test_special_tokens() {
        assert_eq!(normalize("path/to/file.txt"), "path-or-to-or-file.txt");
        assert_eq!(normalize("Ben & Jerry.txt"), "ben-and-jerry.txt");
        assert_eq!(normalize("user@domain.txt"), "user-at-domain.txt");
        assert_eq!(normalize("50% off.txt"), "50-percent-off.txt");
    }

    #[test]
    fn test_transliteration() {
        assert_eq!(normalize("naïve.txt"), "naive.txt");
        assert_eq!(normalize("résumé.pdf"), "resume.pdf");
        assert_eq!(normalize("Björk.mp3"), "bjork.mp3");
        assert_eq!(normalize("façade.jpg"), "facade.jpg");
        assert_eq!(normalize("piñata.txt"), "pinata.txt");
        assert_eq!(normalize("café.txt"), "cafe.txt");
        assert_eq!(normalize("tête-à-tête.txt"), "tete-a-tete.txt");
    }

    #[test]
    fn test_unicode_dashes_and_quotes() {
        // En-dash and em-dash
        assert_eq!(normalize("file–name.txt"), "file-name.txt");
        assert_eq!(normalize("file—name.txt"), "file-name.txt");

        // Curly quotes (using unicode escape sequences)
        assert_eq!(normalize("file\u{2018}name\u{2019}.txt"), "file-name-.txt");
        assert_eq!(normalize("file\u{201C}name\u{201D}.txt"), "file-name-.txt");
    }

    #[test]
    fn test_forbidden_characters() {
        assert_eq!(
            normalize("file#with$special%chars.txt"),
            "file-with-special-percent-chars.txt"
        );
        assert_eq!(
            normalize("file(with)brackets[].txt"),
            "file-with-brackets-.txt"
        );
        assert_eq!(
            normalize("file\\with/slashes.txt"),
            "file-with-or-slashes.txt"
        );
    }

    #[test]
    fn test_custom_config_overrides() {
        let config = NormalizationConfig::from_toml_str(
            r#"
            [special_tokens]
            "&" = "-plus-"

            [transliterations]
            "ø" = "oe"

            [options]
            lowercase = false
            lowercase_extension = false
            "#,
        )
        .expect("config should parse");

        assert_eq!(
            normalize_with_config("Mix&Match.FILE", &config),
            "Mix-plus-Match.FILE"
        );
        assert_eq!(
            normalize_with_config("Smørbrød.txt", &config),
            "Smorbroed.txt"
        );
    }

    #[test]
    fn test_hyphen_cleanup() {
        assert_eq!(
            normalize("file---with--multiple-hyphens.txt"),
            "file-with-multiple-hyphens.txt"
        );
        assert_eq!(normalize("--leading-hyphens.txt"), "leading-hyphens.txt");
        assert_eq!(normalize("multiple   spaces.txt"), "multiple-spaces.txt");
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(normalize(""), "");

        // No extension
        assert_eq!(normalize("filename"), "filename");

        // Only extension
        assert_eq!(normalize(".txt"), ".txt");

        // Hidden file (entire name treated as extension)
        assert_eq!(normalize(".hidden"), ".hidden");
        assert_eq!(normalize(".bashrc"), ".bashrc");

        // Trailing dot (no extension)
        assert_eq!(normalize("filename."), "filename");

        // Already normalized
        assert_eq!(
            normalize("already-normalized.txt"),
            "already-normalized.txt"
        );

        // Whitespace and dots
        assert_eq!(normalize("  .file name. "), "file-name");
        assert_eq!(normalize("...dotted..."), "dotted");
    }

    #[test]
    fn test_extension_handling() {
        assert_eq!(normalize("FILE.TXT"), "file.txt");
        assert_eq!(normalize("file.PDF"), "file.pdf");
        assert_eq!(normalize("archive.tar.gz"), "archive.tar.gz");
        assert_eq!(normalize("script.sh"), "script.sh");
    }
}
