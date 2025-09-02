# TODO - Go Conventions & Best Practices

This file tracks improvements to align the project with Go conventions and best practices.

## Phase 1: Quick Wins (High Impact, Low Effort)

### Performance Improvements

- [ ] **Move regex compilation to package level** (main.go:104, 108)

  ```go
  var (
      forbiddenCharsRe = regexp.MustCompile(`[^a-z0-9\-_.]`)
      multiHyphenRe    = regexp.MustCompile(`-+`)
  )
  ```

- [ ] **Define constants for magic values** (main.go:97, 104)

  ```go
  const (
      spaceReplacer = "-"
      allowedChars  = `[^a-z0-9\-_.]`
  )
  ```

### Documentation (Go Convention)

- [ ] **Add package documentation comment** (top of main.go)

  ```go
  // Package main provides fnorm, a file name normalizer that converts
  // file names to lowercase with consistent formatting rules.
  package main
  ```

- [ ] **Add function documentation** for all functions:
  - [ ] `normalizeFilename` - Core normalization logic
  - [ ] `processFile` - File processing with error handling
  - [ ] `showHelp` - Usage information display

## Phase 2: Core Improvements

### Error Handling

- [ ] **Use error wrapping** for better context (main.go:77, 80)

  ```go
  return fmt.Errorf("target file already exists %q: %w", normalized, os.ErrExist)
  ```

- [ ] **Add validation** in normalizeFilename for edge cases
  - [ ] Handle empty input strings
  - [ ] Handle files starting/ending with dots

### Testing Enhancements

- [ ] **Add edge case tests** (normalize_test.go)
  - [ ] Empty strings: `""` → `""`
  - [ ] Files without extensions: `"README"` → `"readme"`
  - [ ] Multiple dots: `"file.name.txt"` → `"file-name.txt"`
  - [ ] Unicode characters: `"café.txt"` → `"caf-.txt"`
  - [ ] Leading/trailing spaces: `" file "` → `"file"`

- [ ] **Create benchmark tests** (new file: normalize_bench_test.go)

  ```go
  func BenchmarkNormalizeFilename(b *testing.B) {
      for i := 0; i < b.N; i++ {
          normalizeFilename("My Complex File Name (Copy) #1.PDF")
      }
  }
  ```

## Phase 3: Optional Structure Improvements

### Project Layout (Standard Go Project)

- [ ] **Create cmd directory structure**
  - [ ] Move main.go → cmd/fnorm/main.go
  - [ ] Create internal/normalize/ package

- [ ] **Separate CLI from business logic**
  - [ ] Create normalize.go in internal/normalize/
  - [ ] Export Filename function
  - [ ] Keep main.go focused on CLI concerns

### Additional Testing

- [ ] **Add example tests** (example_test.go)

  ```go
  func ExampleNormalizeFilename() {
      result := normalizeFilename("My File.PDF")
      fmt.Println(result)
      // Output: my-file.pdf
  }
  ```

## Phase 4: Enhanced Documentation

### README Updates

- [ ] **Add godoc badge**
- [ ] **Include benchmark results**
- [ ] **Add more usage examples**

### Go Documentation

- [ ] **Create doc.go** for package-level documentation
- [ ] **Add usage examples** in function comments

## Phase 5: Development Quality

### Linting & Quality

- [x] **Add .golangci.yml** for consistent linting
- [ ] **Test with go vet -all**
- [ ] **Add code coverage reporting**

### Build & Distribution

- [ ] **Add version information** with ldflags
- [ ] **Consider adding Dockerfile**
- [ ] **Add GitHub Actions** for CI/CD

---

## Notes

- Each checkbox can be checked off as you complete tasks
- Items are ordered by impact and ease of implementation
- Feel free to tackle them in any order that makes sense
- Update CLAUDE.md and README.md when you make structural changes
