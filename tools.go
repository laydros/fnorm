//go:build tools
// +build tools

// tools.go documents development tool dependencies.
// The tools build tag ensures this file is ignored in normal builds.
// Tools are installed via `make tools` which uses `go install` with @latest.
package main

// Development tools used by this project:
//
//   golangci-lint: Static analysis and linting
//     Installation: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
