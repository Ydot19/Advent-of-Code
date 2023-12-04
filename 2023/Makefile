BIN_DIR := $(PWD)/bin
CMD_DIR := $(PWD)/cmd
DAY?=1
OUTPUT := $(BIN_DIR)/day$(DAY)
TARGET := $(CMD_DIR)/day$(DAY)

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean

.PHONY: generate-binary
generate-binary:
	$(GOBUILD) -o $(OUTPUT) $(TARGET)
	chmod +x $(OUTPUT)
