# fnorm - File name normalizer

BINARY_NAME=fnorm
BINARY_UNIX=$(BINARY_NAME)_unix
BINARY_WINDOWS=$(BINARY_NAME).exe

# Version from git or fallback
VERSION ?= $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")
LDFLAGS=-ldflags "-X main.version=$(VERSION)"

.DEFAULT_GOAL := build

build:
	go build $(LDFLAGS) -o $(BINARY_NAME) -v

clean:
	go clean
	rm -f $(BINARY_NAME)
	rm -f $(BINARY_UNIX)
	rm -f $(BINARY_WINDOWS)

test:
	go test -v

coverage:
	go test -coverprofile=coverage.out ./...

run:
	go run . $(ARGS)

# Development tools
tools:
	go mod download
	go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

# Code quality checks
lint:
	golangci-lint run

fmt:
	go fmt ./...

vet:
	go vet -all ./...

check: fmt vet lint test

# Cross compilation
build-linux:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build $(LDFLAGS) -o $(BINARY_UNIX) -v

build-windows:
	CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build $(LDFLAGS) -o $(BINARY_WINDOWS) -v

build-all: build build-linux build-windows

install: build
	cp $(BINARY_NAME) ~/bin/

.PHONY: build clean test coverage run tools lint fmt vet check build-linux build-windows build-all install

