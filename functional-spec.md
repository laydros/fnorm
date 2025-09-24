# fnorm Functional Specification

This document defines the externally observable behavior of the **fnorm** filename normalization tool and its accompanying Go library. It is intended to be sufficient for an independent implementation of the same functionality in another language or environment.

## 1. Overview

*Purpose*: Convert one or more filesystem paths so that the file names conform to a normalized, ASCII-only slug format while retaining their directory location and file extensions. The program can be used as a command-line renaming utility or as a library function that returns the normalized name as a string.

*Scope*: Only filename text is transformed; file contents are never modified. Directory names supplied as command arguments are rejected. Symbolic links are treated according to the underlying filesystem behavior of `os.Stat` (i.e., they are dereferenced).

## 2. Command-Line Interface

### 2.1 Invocation Syntax

```shell
fnorm [flags] file1 [file2 ...]
```

* `file1 … fileN` – One or more file paths. Relative and absolute paths are accepted. Globs are expanded by the invoking shell, not by `fnorm`.

### 2.2 Flags

| Flag | Effect |
|------|--------|
| `-dry-run` | Prints the rename that would occur for each file but leaves the filesystem unchanged. |
| `-version` | Writes `fnorm version <value>` to standard output and exits with status 0 without processing file arguments. |
| `-h`, `--help` | Emits the usage/help text produced by Go's `flag` package, which contains a description of the normalization rules, flag list, and examples. Exits with status 0. |

No other flags are recognized. An unknown flag causes the standard Go `flag` parser error before `main` executes custom logic.

### 2.3 Exit Status

| Status | Meaning |
|--------|---------|
| `0` | All supplied paths were processed successfully (including dry-run). |
| `1` | At least one path failed to process (e.g., file missing, directory argument, target filename collision). |

The utility attempts to process every provided argument even when some fail; it only reports a non-zero exit after all paths have been attempted.

### 2.4 Output Streams

* **Standard output** – Success messages and informational output:
  * `fnorm version <value>` for `-version`.
  * `Renamed: <old> -> <new>` when a rename occurs.
  * `✓ <name> (no changes needed)` when a filename is already normalized and `-dry-run` is not set.
  * `Would rename: <old> -> <new>` for `-dry-run` renames.
  * Dry-run mode does **not** emit a message for files that already satisfy the normalization rules.
* **Standard error** – Diagnostic messages:
  * When no positional arguments are supplied: prints `Error: No files specified` followed by a reminder to use help and exits with status 1.
  * When processing a file fails: prints `Error processing <path>: <detailed message>` and continues with remaining arguments. The detailed message matches the error returned by the filesystem operation (e.g., `stat ...: no such file or directory`, `skipping directory ...: is a directory`, `target file already exists "<name>": file exists`).

### 2.5 File Processing Algorithm

For each file argument after flag parsing:

1. Call `os.Stat` on the supplied path.
   * If the call fails (missing file, permission error, etc.), record the error, report it to stderr, and skip further work for that argument.
   * If the path refers to a directory, report an error `skipping directory <path>: is a directory` and mark the operation as failed.
2. Compute the normalized filename by applying the transformation rules in Section 3 to the basename (the directory component is preserved).
3. Determine the rename strategy:
   * If the normalized name is identical to the original name:
     * In normal mode, print `✓ <name> (no changes needed)` to stdout.
     * In dry-run mode, print nothing.
   * If a change is required and `-dry-run` is active, emit `Would rename: <old> -> <new>` and skip filesystem changes.
   * If a change is required and `-dry-run` is not active:
     * Detect case-only renames by comparing the old and new full paths with `strings.EqualFold`. For case-only renames, perform a two-step rename via a temporary `<original>.fnorm-tmp` filename to support case-insensitive filesystems. Restore the original name if the second step fails.
     * For other renames, fail early if a file already exists at the target path (`os.Stat` check) and report `target file already exists`.
     * Apply `os.Rename` to move the file. Upon success, print `Renamed: <old> -> <new>`.
4. After all arguments are processed, exit with code 1 if any of the operations returned an error; otherwise exit 0.

## 3. Filename Normalization Rules

The library function `fnorm.Normalize(string) string` performs the following deterministic transformation. The CLI uses the same function internally.

1. **Empty input** – Returns the empty string immediately.
2. **Extension detection** – Determine the extension with Go's `filepath.Ext`. The extension is the substring from the final `.` to the end of the string. A lone trailing dot (`."`) is treated as no extension. The remainder before the extension becomes the *base name*.
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
* Names beginning with `.` that do not contain additional dots (e.g., `.bashrc`) are treated as extensions by `filepath.Ext`; the function therefore lowercases the entire name without applying base-name rules. Example: `.Hidden File` becomes `.hidden file`, preserving spaces. This is a known deviation from the intended hyphenation behavior.

## 4. Library API Contract

```
package fnorm
func Normalize(filename string) string
```

* Pure function: produces the same output for the same input and has no side effects.
* Accepts any UTF-8 string and returns a normalized filename per Section 3.
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
| `.Hidden File` | `.hidden file` (spaces preserved because the entire name is treated as an extension) |

## 6. Error Conditions Summary

| Condition | Behavior |
|-----------|----------|
| No positional arguments | Prints error about missing files, exit status 1. |
| Argument refers to a directory | Prints `Error processing <path>: skipping directory <path>: is a directory`, exit status reflects failure. |
| Argument path cannot be stat'ed | Prints `Error processing <path>: stat <path>: <system error>`, marks failure. |
| Target normalized filename already exists (non case-only) | Prints `Error processing <path>: target file already exists "<normalized>": file exists`, marks failure. |
| Rename syscall failure | Prints `Error processing <path>: failed to rename ...`, marks failure. |

## 7. Determinism and Idempotence

Running `fnorm` multiple times on the same set of files is idempotent: after the first successful rename, subsequent runs either report `no changes needed` (normal mode) or remain silent (dry-run mode) with exit status 0. The normalization rules themselves are deterministic for a given input string.

## 8. Platform Considerations

* Case-insensitive filesystems are supported through the two-step rename strategy described in Section 2.5. Case-only changes succeed without requiring manual intervention.
* The program relies on the operating system for permission checks and error reporting; no retries are attempted beyond the case-only rename workflow.

## 9. Known Limitations

* Hidden files whose names consist solely of a leading dot followed by characters without another dot are not fully normalized: hyphenation and forbidden character replacement are not applied because the entire name is interpreted as the extension. Example: `.Hidden File` → `.hidden file` (space preserved).
* The transliteration table is limited to the explicit runes listed in Section 3.7; other Unicode characters are reduced to hyphens by the forbidden-character filter.
