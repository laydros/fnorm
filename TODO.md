# TODO - Go Conventions & Best Practices

This file tracks improvements to align the project with Go conventions and best practices.

## Phase 1: Quick Wins (High Impact, Low Effort)

### Performance Improvements

- [x] **Move regex compilation to package level** (main.go:104, 108) ✅ _Completed - moved to package-level vars_

  ```go
  var (
      forbiddenCharsRe = regexp.MustCompile(forbiddenCharsPattern)
      multiHyphenRe    = regexp.MustCompile(`-+`)
  )
  ```

- [x] **Define constants for magic values** (main.go:97, 104) ✅ _Completed - added const section with proper naming_

  ```go
  const (
      spaceReplacer         = "-"
      forbiddenCharsPattern = `[^a-z0-9\-_.]`
  )
  ```

### Documentation (Go Convention)

- [x] **Add package documentation comment** (top of main.go) ✅ _Completed in commit 73f4024_

  ```go
  // Package main provides fnorm, a file name normalizer that converts
  // file names to lowercase with consistent formatting rules.
  package main
  ```

- [x] **Add function documentation** for all functions: ✅ _Completed - added proper Go doc comments_
  - [x] `Normalize` - Core normalization logic
  - [x] `processFile` - File processing with error handling
  - [x] `showHelp` - Usage information display

## Phase 2: Core Improvements

### Error Handling

- [x] **Use error wrapping** for better context (main.go:77, 80)

  ```go
  return fmt.Errorf("target file already exists %q: %w", normalized, os.ErrExist)
  ```

- [x] **Add validation** in Normalize for edge cases
  - [x] Handle empty input strings
  - [x] Handle files starting/ending with dots

### Testing Enhancements

- [x] **Add edge case tests** (normalize_test.go)
  - [x] Empty strings: `""` → `""`
  - [x] Files without extensions: `"README"` → `"readme"`
  - [x] Multiple dots: `"file.name.txt"` → `"file.name.txt"`
  - [x] Unicode characters: `"café.txt"` → `"cafe.txt"`
  - [x] Leading/trailing spaces: `" file "` → `"file"`

- [x] **Create benchmark tests** (new file: normalize_bench_test.go)

  ```go
  func BenchmarkNormalize(b *testing.B) {
      for i := 0; i < b.N; i++ {
          Normalize("My Complex File Name (Copy) #1.PDF")
      }
  }
  ```

## Phase 3: Optional Structure Improvements

### Project Layout (Standard Go Project)

- [x] **Create cmd directory structure**
  - [x] Move main.go → cmd/fnorm/main.go
  - [x] Create internal/normalize/ package

- [x] **Separate CLI from business logic**
  - [x] Create normalize.go in internal/normalize/
  - [x] Export Normalize function
  - [x] Keep main.go focused on CLI concerns

### Additional Testing

- [x] **Add example tests** (example_test.go)

  ```go
  func ExampleNormalize() {
      result := Normalize("My File.PDF")
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

- [x] **Add .golangci.yml** for consistent linting ✅ _Completed in commits 73f4024 & a5fe935_
- [ ] **Test with go vet -all**
- [ ] **Add code coverage reporting**

### Build & Distribution

- [x] **Add version information** with ldflags ✅ _Completed - added -version flag and Makefile ldflags injection_
- [ ] **Consider adding Dockerfile**
- [x] **Add GitHub Actions** for CI/CD ✅ _Completed in commit 7cc7ee7_

---

## ✅ Recently Completed (Latest Commits)

### Development Infrastructure (Phase 5)

- **GitHub Actions CI/CD** - Automated testing and linting on push/PR
- **golangci-lint Configuration** - Comprehensive linting rules with schema compliance
- **Development Tooling Setup** - `tools.go`, enhanced Makefile with quality checks
- **Package Documentation** - Added required package comment
- **Updated Documentation** - README.md and CLAUDE.md reflect new tooling

### Files Added/Modified

- `.github/workflows/ci.yml` - CI pipeline
- `.golangci.yml` - Linter configuration
- `tools.go` - Development dependencies
- `Makefile` - Enhanced with `make tools`, `make check`, etc.
- `main.go` - Added package documentation
- `README.md` & `CLAUDE.md` - Updated development setup

---

## Notes

- Each checkbox can be checked off as you complete tasks
- Items are ordered by impact and ease of implementation
- Feel free to tackle them in any order that makes sense
- Update CLAUDE.md and README.md when you make structural changes
- ✅ = Recently completed tasks (see commit history)
