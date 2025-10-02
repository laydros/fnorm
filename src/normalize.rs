/// Normalize a filename according to the fnorm rules
#[must_use]
pub fn normalize(filename: &str) -> String {
    // Step 1: Empty input
    if filename.is_empty() {
        return String::new();
    }

    // Step 2: Extension detection
    let (base_name, extension) = split_extension(filename.trim());

    // Steps 3-10: Normalization pipeline in a single pass for the base name
    let base = normalize_base(base_name);

    // Step 11: Extension normalization
    let normalized_extension = extension.to_lowercase();

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

fn normalize_base(base_name: &str) -> String {
    let trimmed = base_name.trim().trim_matches('.');

    if trimmed.is_empty() {
        return String::new();
    }

    let mut processed = String::with_capacity(trimmed.len() * 2);

    for ch in trimmed.chars() {
        for lower in ch.to_lowercase() {
            match lower {
                '/' => processed.push_str("-or-"),
                '&' => processed.push_str("-and-"),
                '@' => processed.push_str("-at-"),
                '%' => processed.push_str("-percent-"),
                'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => processed.push('a'),
                'é' | 'è' | 'ê' | 'ë' => processed.push('e'),
                'í' | 'ì' | 'î' | 'ï' => processed.push('i'),
                'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ø' => processed.push('o'),
                'ú' | 'ù' | 'û' | 'ü' => processed.push('u'),
                'ñ' => processed.push('n'),
                'ç' => processed.push('c'),
                'æ' => processed.push_str("ae"),
                'œ' => processed.push_str("oe"),
                'ß' => processed.push_str("ss"),
                ' ' | '–' | '—' | '\u{2018}' | '\u{2019}' | '\u{201C}' | '\u{201D}' => {
                    processed.push('-');
                }
                _ => {
                    if lower.is_ascii_lowercase()
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
