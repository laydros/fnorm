# fnorm Functional Specification

This document defines the externally observable behavior of the **fnorm** filename normalization tool and its accompanying Rust library. It is intended to be sufficient for an independent implementation of the same functionality in another language or environment.

## 1. Overview

*Purpose*: Convert one or more filesystem paths so that the file and directory names conform to a normalized, ASCII-only slug format while retaining their directory location and file extensions. The program can be used as a command-line renaming utility or as a library function that returns the normalized name as a string.

*Scope*: Only the name of each supplied path is transformed; file contents are never modified. Directories supplied as arguments are renamed like files, and their contents are left untouched; there is no recursion. Symbolic links are followed when checking that a path exists, and the link itself is renamed.

## 2. Command-Line Interface

### 2.1 Invocation Syntax

```shell
fnorm [OPTIONS] <FILE>...
```

* `<FILE>...` – One or more file or directory paths. At least one is required. Relative and absolute paths are accepted. Globs are expanded by the invoking shell, not by `fnorm`.

### 2.2 Flags

| Flag | Effect |
|------|--------|
| `--dry-run` | Prints the rename that would occur for each file but leaves the filesystem unchanged. |
| `--config <path>` | Loads a TOML file that overrides the default normalization rules. The file format is documented in the README. |
| `-V`, `--version` | Writes `fnorm <version>` to standard output and exits with status 0 without processing file arguments. |
| `-h`, `--help` | Prints usage, the argument list and the option list, then exits with status 0. |

No other flags are recognized. An unknown flag, or no file arguments, prints a usage error to standard error and exits with status 2 before any path is processed.

### 2.3 Exit Status

| Status | Meaning |
|--------|---------|
| `0` | All supplied paths were processed successfully (including dry-run). |
| `1` | At least one path failed to process (e.g., file missing, target filename collision), or the config file could not be read or parsed. |
| `2` | Usage error: unknown flag or no file arguments. |

The utility attempts to process every provided argument even when some fail; it only reports a non-zero exit after all paths have been attempted.

### 2.4 Output Streams

* **Standard output** – Success messages and informational output:
  * `fnorm <version>` for `--version`.
  * `Renamed: <old> -> <new>` when a rename occurs.
  * `✓ <name> (no changes needed)` when a filename is already normalized and `--dry-run` is not set.
  * `Would rename: <old> -> <new>` for `--dry-run` renames.
  * Dry-run mode does **not** emit a message for files that already satisfy the normalization rules.
* **Standard error** – Diagnostic messages:
  * When no file arguments are supplied: prints `error: the following required arguments were not provided`, the usage line, and a pointer to `--help`, then exits with status 2.
  * When processing a file fails, the error is recorded and processing continues with the remaining arguments. After all arguments have been attempted, a single summary is printed:

    ```
    failed to process <N> path(s):
      <path>: <detailed message>
        caused by: <system error>
    ```

    One `<path>: <detailed message>` line is printed per failed argument; the `caused by:` line appears only when an underlying OS error is available. Detailed messages are `file not found` (the path does not exist), `cannot access path` (any other failure to read the path, such as a permission error), `target file already exists: "<target>"`, and `failed to rename "<from>" to "<to>"`.
  * When the configuration file cannot be read or parsed: prints `failed to read config at <path>` or `failed to parse config at <path>`, followed by `  caused by: <reason>` (the OS error or the TOML parse error), and exits with status 1 without processing any paths. A config key that is not a single character prints `invalid key "<key>" in <section>; use single-character keys` and exits the same way.

### 2.5 File Processing Algorithm

For each file argument after flag parsing:

1. Check that the supplied path exists, following symbolic links.
   * If the check fails (missing file, permission error, etc.), record the error and skip further work for that argument. The error is reported in the summary at the end.
   * Files and directories are handled the same way.
2. Compute the normalized filename by applying the transformation rules in Section 3 to the basename (the directory component is preserved).
3. Determine the rename strategy:
   * If the normalized name is identical to the original name:
     * In normal mode, print `✓ <name> (no changes needed)` to stdout.
     * In dry-run mode, print nothing.
   * If a change is required and `--dry-run` is active, emit `Would rename: <old> -> <new>` and skip filesystem changes.
   * If a change is required and `--dry-run` is not active:
     * Detect case-only renames by comparing the lowercased old and new names. For case-only renames, fail with `target file already exists` if the directory already contains an entry with the exact new name (a distinct file on a case-sensitive filesystem) or the temporary name; otherwise perform a two-step rename via a temporary `<original>.fnorm-tmp` filename to support case-insensitive filesystems. Restore the original name if the second step fails.
     * For other renames, fail early if anything, including a dangling symlink, already exists at the target path (checked without following symbolic links) and report `target file already exists`.
     * Rename the path. Upon success, print `Renamed: <old> -> <new>`.
4. After all arguments are processed, exit with code 1 if any of the operations returned an error; otherwise exit 0.

## 3. Filename Normalization Rules

The library function `fnorm::normalize(&str) -> String` performs the following deterministic transformation. The CLI uses the same function internally. These are the default rules; `--config` can change the replacement tables and whether the extension is lowercased (see the README).

1. **Empty input** – Returns the empty string immediately.
2. **Extension detection** – The extension is the substring from the final `.` to the end of the string. A lone trailing dot (`."`) is treated as no extension. The remainder before the extension becomes the *base name*.
3. **Whitespace and dot trimming (base name only)** – Remove leading/trailing ASCII whitespace, then strip leading and trailing literal periods `.` from the base name. Interior dots are preserved.
4. **Space replacement (base name only)** – Replace each literal space U+0020 with `-`.
5. **Lowercasing (base name only)** – Convert the base name to lowercase using Unicode simple case folding.
6. **Special token substitution (base name only)** – Replace the exact characters `/`, `&`, `@`, `%` with `-or-`, `-and-`, `-at-`, and `-percent` respectively. Replacements occur wherever the characters appear, even when introduced by earlier steps.
7. **Transliteration (base name only)** – Replace each rune using the table below; characters not listed are left unchanged. The process is character-wise and not context-aware.

   | Source runes | Replacement |
   |--------------|-------------|
   | `á à â ä ã å` | `a` |
   | `é è ê ë` | `e` |
   | `í ì î ï` | `i` |
   | `ó ò ô ö õ` | `o` |
   | `ú ù û ü` | `u` |
   | `ñ` | `n` |
   | `ç` | `c` |
   | `æ` | `ae` |
   | `œ` | `oe` |
   | `ø` | `o` |
   | `ß` | `ss` |
   | `– —` (en/em dash) | `-` |
   | `‘ ’` (curly single quotes) | `'` |
   | `“ ”` (curly double quotes) | `"` |

8. **Forbidden character filtering (base name only)** – Replace every character that is not a lowercase ASCII letter `a–z`, digit `0–9`, hyphen `-`, underscore `_`, or period `.` with `-`.
9. **Hyphen cleanup (base name only)** – Collapse runs of one or more consecutive hyphens into a single `-`.
10. **Leading hyphen trim (base name only)** – Remove any remaining leading hyphen characters.
11. **Extension normalization** – Convert the extension (if any) to lowercase. No other transformations are applied to the extension portion.
12. **Reassembly** – Concatenate the processed base name with the (possibly empty) lowercase extension and return the result.

### 3.1 Resulting Character Set

After normalization, the filename will consist solely of lowercase ASCII letters, digits, hyphen, underscore, and period. Periods may separate the base name from the extension or remain in the base if originally present and permitted by the filtering rules. Hyphens never appear in sequence because of step 9.

### 3.2 Behavior of Special Cases

* Filenames lacking an extension (no `.` after the first character) are returned as the normalized base name without a trailing dot.
* A filename that is already compliant (e.g., `example-file.txt`) returns unchanged.
* An input of `""` yields `""`.
* Names beginning with `.` are hidden files. Leading dots are a hidden-file marker, not an extension delimiter: they are removed, the remainder is normalized with the full rules (including extension detection), and a single `.` is reattached. Examples: `.Hidden File` → `.hidden-file`, `.Hidden File.TXT` → `.hidden-file.txt`, `.bashrc` → `.bashrc`. If the remainder normalizes to an empty string, the result is `""`.

## 4. Library API Contract

```rust
pub fn normalize(filename: &str) -> String
pub fn normalize_with_config(filename: &str, config: &NormalizationConfig) -> String
```

* Pure function: produces the same output for the same input and has no side effects.
* Accepts any UTF-8 string and returns a normalized filename per Section 3. `normalize_with_config` applies the same steps with a custom `NormalizationConfig`.
* Intended for consumer code that wants to derive a normalized string without performing filesystem operations.

## 5. Examples

| Input | Output |
|-------|--------|
| `My Document.PDF` | `my-document.pdf` |
| `File & Video.mov` | `file-and-video.mov` |
| `tcp/udp guide.md` | `tcp-or-udp-guide.md` |
| `café menu.txt` | `cafe-menu.txt` |
| `rock’n’roll.txt` | `rock-n-roll.txt` |
| `Résumé` | `resume` |
| `.Hidden File` | `.hidden-file` (leading dot preserved, remainder normalized) |

## 6. Error Conditions Summary

| Condition | Behavior |
|-----------|----------|
| No file arguments | Prints a usage error, exit status 2. |
| Argument path does not exist | Reports `<path>: file not found` (with `caused by: <system error>`) in the summary, marks failure. |
| Argument path cannot be read (e.g., permission denied) | Reports `<path>: cannot access path` (with `caused by: <system error>`) in the summary, marks failure. |
| Target normalized filename already exists | Reports `<path>: target file already exists: "<target>"` in the summary, marks failure. |
| Rename syscall failure | Reports `<path>: failed to rename "<from>" to "<to>"` (with `caused by: <system error>`) in the summary, marks failure. |

## 7. Determinism and Idempotence

Running `fnorm` multiple times on the same set of files is idempotent: after the first successful rename, subsequent runs either report `no changes needed` (normal mode) or remain silent (dry-run mode) with exit status 0. The normalization rules themselves are deterministic for a given input string.

## 8. Platform Considerations

* Case-insensitive filesystems are supported through the two-step rename strategy described in Section 2.5. Case-only changes succeed without requiring manual intervention.
* The program relies on the operating system for permission checks and error reporting; no retries are attempted beyond the case-only rename workflow.

## 9. Known Limitations

* The transliteration table is limited to the explicit runes listed in Section 3, step 7 (plus any added by `--config`); other Unicode characters are reduced to hyphens by the forbidden-character filter.
