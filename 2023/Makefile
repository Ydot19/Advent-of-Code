BIN_DIR := $(PWD)/bin
CMD_DIR := $(PWD)/cmd
DAY?=1
OUTPUT := $(BIN_DIR)/day$(DAY)_go
TARGET := $(CMD_DIR)/day$(DAY)

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean

# Rust parameters
RUSTCMD=cargo
RUSTBUILD=$(RUSTCMD) build --release --target-dir=$(BIN_DIR)

.PHONY: generate-binary
go-generate-binary:
	$(GOBUILD) -o $(OUTPUT) $(TARGET)
	chmod +x $(OUTPUT)

rust-generate-binary:
	$(RUSTBUILD) 


