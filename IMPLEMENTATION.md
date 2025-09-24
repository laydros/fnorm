# fnorm Implementation Progress

This document tracks the implementation progress of the fnorm Rust port from Go. It serves as a checkpoint for development work and helps maintain continuity across sessions.

## Current Status

**Phase**: Initial Rust port structure created, normalization logic not yet implemented
**Last Updated**: 2025-09-24

### Completed ✅

1. **Project Structure**
   - Cargo.toml configured with clap and anyhow dependencies
   - Basic module structure (main.rs, normalize.rs, error.rs)
   - CLI argument parsing with clap (--dry-run flag)
   - Error types defined in error.rs

2. **Placeholder Implementation**
   - Basic CLI that accepts file arguments
   - File existence and directory checking
   - Placeholder normalize function (currently just lowercases and replaces spaces with underscores)
   - Basic test structure

3. **Documentation**
   - functional-spec.md defines complete behavior
   - CLAUDE.md provides project guidance
   - Basic README exists

### In Progress 🚧

None currently active.

### To Do 📋

#### Step 1: Core Normalization Logic
Implement the 12-step normalization algorithm in `src/normalize.rs`:
- [ ] Extension detection using Rust equivalent of filepath.Ext
- [ ] Whitespace and dot trimming (base name only)
- [ ] Space replacement with hyphens
- [ ] Lowercasing with Unicode support
- [ ] Special token substitution (/, &, @, %)
- [ ] Character transliteration table
- [ ] Forbidden character filtering
- [ ] Hyphen cleanup (collapse consecutive)
- [ ] Leading hyphen removal
- [ ] Extension normalization
- [ ] Reassembly of base name and extension

#### Step 2: Comprehensive Tests
Update tests in `src/normalize.rs`:
- [ ] All examples from functional spec
- [ ] Hidden file edge cases
- [ ] Files without extensions
- [ ] Special character handling
- [ ] Transliteration verification
- [ ] Multiple hyphen collapse

#### Step 3: Main Process Integration
Update `src/main.rs`:
- [ ] Call normalize function on file basenames
- [ ] Implement actual file renaming
- [ ] Handle dry-run mode properly
- [ ] Case-only rename detection for case-insensitive filesystems
- [ ] Proper output messages per spec
- [ ] Error aggregation and exit codes

#### Step 4: Real-world Testing
- [ ] Create test files with various naming patterns
- [ ] Verify case-insensitive filesystem handling
- [ ] Test with directories (should error)
- [ ] Test with existing target files

## Implementation Notes

### Key Rust Considerations

1. **Path Handling**: Use `std::path::Path` and `PathBuf` for file operations
2. **Extension Detection**: Use `Path::extension()` and `Path::file_stem()`
3. **Unicode Support**: Rust strings are UTF-8 by default, use `.chars()` for character iteration
4. **Case Folding**: Use `.to_lowercase()` for simple case folding
5. **String Building**: Consider using `String::with_capacity()` for performance

### Differences from Go Implementation

- Rust uses `Result<T, E>` for error handling instead of Go's multiple return values
- File operations use `std::fs` instead of Go's `os` package
- Pattern matching with `match` instead of Go's switch statements
- Ownership and borrowing considerations for string manipulation

### Testing Strategy

Run tests with:
```bash
cargo test
cargo test -- --nocapture  # To see println! output
```

### Build and Run

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run with files
cargo run -- file1.txt file2.pdf

# Run in dry-run mode
cargo run -- --dry-run "My Document.PDF"
```

## Next Session Checklist

When resuming development:
1. Check this document for current status
2. Review functional-spec.md for requirements
3. Run `cargo test` to verify current state
4. Continue with the next uncompleted task

## Known Issues

1. The placeholder normalize function uses underscores instead of hyphens (typo to fix)
2. CI workflow still references Go commands (needs update after port complete)
3. Hidden file handling needs special attention per functional spec deviation