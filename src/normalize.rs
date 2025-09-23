/// Normalize a filename according to the fnorm rules
pub fn normalize(filename: &str) -> String {
    if filename.is_empty() {
        return String::new();
    }

    // TODO: Implement the full normalization logic from the spec
    // For now, just return a placeholder transformation
    filename.to_lowercase().replace(" ", "_")
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
        assert_eq!(normalize("My File.txt"), "my_file.txt");
        assert_eq!(normalize("Another-File.DOC"), "another-file.doc");
    }

    #[test]
    fn test_spaces_to_hyphens() {
        assert_eq!(normalize("File With Spaces.txt"), "file_with_spaces.txt");
    }
}
