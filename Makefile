# fnorm - File name normalizer

BINARY_NAME=fnorm
BINARY_UNIX=$(BINARY_NAME)_unix
BINARY_WINDOWS=$(BINARY_NAME).exe

.DEFAULT_GOAL := build

build:
	go build -o $(BINARY_NAME) -v

clean:
	go clean
	rm -f $(BINARY_NAME)
	rm -f $(BINARY_UNIX)
	rm -f $(BINARY_WINDOWS)

test:
	go test -v

run:
	go run . $(ARGS)

# Cross compilation
build-linux:
	CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o $(BINARY_UNIX) -v

build-windows:
	CGO_ENABLED=0 GOOS=windows GOARCH=amd64 go build -o $(BINARY_WINDOWS) -v

build-all: build build-linux build-windows

install: build
	cp $(BINARY_NAME) ~/bin/

.PHONY: build clean test run build-linux build-windows build-all install

