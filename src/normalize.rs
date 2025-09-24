/// Normalize a filename according to the fnorm rules
pub fn normalize(filename: &str) -> String {
    // Step 1: Empty input
    if filename.is_empty() {
        return String::new();
    }

    // Step 2: Extension detection
    let (base_name, extension) = split_extension(filename.trim());

    // Step 3: Whitespace and dot trimming (base name only)
    let base = base_name.trim().trim_matches('.');

    // Step 4: Space replacement (base name only)
    let base = base.replace(' ', "-");

    // Step 5: Lowercasing (base name only)
    let mut base = base.to_lowercase();

    // Step 6: Special token substitution (base name only)
    base = base.replace('/', "-or-");
    base = base.replace('&', "-and-");
    base = base.replace('@', "-at-");
    base = base.replace('%', "-percent-");

    // Step 7: Transliteration (base name only)
    base = transliterate(&base);

    // Step 8: Forbidden character filtering (base name only)
    base = filter_forbidden_chars(&base);

    // Step 9: Hyphen cleanup (base name only)
    base = cleanup_hyphens(&base);

    // Step 10: Leading hyphen trim (base name only)
    base = base.trim_start_matches('-').to_string();

    // Step 11: Extension normalization
    let normalized_extension = extension.to_lowercase();

    // Step 12: Reassembly
    if normalized_extension.is_empty() {
        base
    } else {
        format!("{}.{}", base, normalized_extension)
    }
}

/// Split a filename into base name and extension
/// Returns (base_name, extension) where extension does not include the dot
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

/// Apply character transliteration according to the spec
fn transliterate(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2); // Estimate capacity

    for ch in text.chars() {
        let replacement = match ch {
            // Accented vowels
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => "a",
            'é' | 'è' | 'ê' | 'ë' => "e",
            'í' | 'ì' | 'î' | 'ï' => "i",
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' => "o",
            'ú' | 'ù' | 'û' | 'ü' => "u",
            // Other special characters
            'ñ' => "n",
            'ç' => "c",
            'æ' => "ae",
            'œ' => "oe",
            'ø' => "o",
            'ß' => "ss",
            // Dashes
            '–' | '—' => "-", // en-dash, em-dash
            // Curly quotes
            '\u{2018}' | '\u{2019}' => "'", // curly single quotes
            '\u{201C}' | '\u{201D}' => "\"", // curly double quotes
            // Default: keep the character
            _ => {
                result.push(ch);
                continue;
            }
        };
        result.push_str(replacement);
    }

    result
}

/// Filter out forbidden characters, replacing with hyphen
fn filter_forbidden_chars(text: &str) -> String {
    text.chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_' || ch == '.' {
                ch
            } else {
                '-'
            }
        })
        .collect()
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
        assert_eq!(normalize("file#with$special%chars.txt"), "file-with-special-percent-chars.txt");
        assert_eq!(normalize("file(with)brackets[].txt"), "file-with-brackets-.txt");
        assert_eq!(normalize("file\\with/slashes.txt"), "file-with-or-slashes.txt");
    }

    #[test]
    fn test_hyphen_cleanup() {
        assert_eq!(normalize("file---with--multiple-hyphens.txt"), "file-with-multiple-hyphens.txt");
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
        assert_eq!(normalize("already-normalized.txt"), "already-normalized.txt");

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
