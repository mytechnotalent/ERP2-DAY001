#
# @file Makefile
# @brief Build and test automation for RP2350 blink firmware
# @author Kevin Thomas
# @date 2025
#
# MIT License
#
# Copyright (c) 2025 Kevin Thomas
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#

# Detect host architecture
HOST_TRIPLE := $(shell rustc --version --verbose | grep 'host:' | awk '{print $$2}')

# Default target
.PHONY: all
all: test build

# Run library tests on host architecture
.PHONY: test
test:
	@echo "Running tests on host target: $(HOST_TRIPLE)"
	cargo test --lib --target $(HOST_TRIPLE) --no-default-features

# Build for RP2350 target
.PHONY: build
build:
	@echo "Building for RP2350 target..."
	cargo build --release

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Check code without building
.PHONY: check
check:
	cargo check

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Run clippy linter
.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

# Show help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all     - Run tests and build (default)"
	@echo "  test    - Run library tests on host"
	@echo "  build   - Build for RP2350 target"
	@echo "  clean   - Clean build artifacts"
	@echo "  check   - Check code without building"
	@echo "  fmt     - Format code"
	@echo "  clippy  - Run clippy linter"
	@echo "  help    - Show this help message"
