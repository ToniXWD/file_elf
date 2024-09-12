# Variables
PUBLISH_DIR := ./publish
CARGO_TARGET_DIR := ./target/release
TAURI_APP_DIR := ./app/search-files-app
TAURI_TARGET_DIR := $(TAURI_APP_DIR)/src-tauri/target/release
RUST_BINARY := file_elf
TAURI_BINARY := search-files-app
ZIP_FILE := release_package.zip

# Default target: build everything
.PHONY: all
all: elf tauri

# Build the Rust project
.PHONY: elf
elf:
	@echo "Building Rust file_elf project..."
	cargo build --release

# Build the Tauri app
.PHONY: tauri
tauri:
	@echo "Building Tauri app..."
	cd $(TAURI_APP_DIR) && cargo tauri build

# Ensure the publish directory exists
.PHONY: publish-dir
publish-dir:
	@mkdir -p $(PUBLISH_DIR)

# publish the build artifacts to the publish directory
.PHONY: publish
publish: publish-dir
	@echo "Copying files to the publish directory..."
	cp $(CARGO_TARGET_DIR)/$(RUST_BINARY) $(PUBLISH_DIR)/
	cp ./base.toml $(PUBLISH_DIR)/
	cp $(TAURI_TARGET_DIR)/$(TAURI_BINARY) $(PUBLISH_DIR)/
	@echo "Packaging files into $(ZIP_FILE)..."
	cd $(PUBLISH_DIR) && zip -r ../$(ZIP_FILE) .

# Clean the build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf $(PUBLISH_DIR)
	rm -rf $(TAURI_APP_DIR)/build/
	rm -rf $(TAURI_APP_DIR)/src-tauri/target/

# If you want to only clean the publish directory
.PHONY: clean-publish
clean-publish:
	@echo "Cleaning the publish directory..."
	rm -rf $(PUBLISH_DIR)

.PHONY: clean-tauri
clean-tauri:
	@echo "Cleaning the tauri..."
	rm -rf $(TAURI_APP_DIR)/build/
	rm -rf $(TAURI_APP_DIR)/src-tauri/target/

.PHONY: clean-elf
clean-elf:
	@echo "Cleaning the Rust file_elf project..."
	cargo clean
