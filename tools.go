//go:build tools
// +build tools

// tools.go manages development tool dependencies.
// This file is used to track tool dependencies with `go mod`.
// The tools build tag ensures this file is ignored in normal builds.
package main

import (
	// Static analysis and linting tools
	_ "github.com/golangci/golangci-lint/cmd/golangci-lint"
)
