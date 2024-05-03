# Define variables
PROJECT_NAME := roguelike
WASM_SRC_DIR := target/wasm32-unknown-unknown/release
WASM_BUILD_DIR := wasm

# Define default target
all: build bindgen rename

# Define commands
build:
	cargo build --release --target wasm32-unknown-unknown

bindgen:
	wasm-bindgen $(WASM_SRC_DIR)/$(PROJECT_NAME).wasm --out-dir $(WASM_BUILD_DIR) --no-modules --no-typescript

rename:
	mv $(WASM_BUILD_DIR)/$(PROJECT_NAME).js $(WASM_BUILD_DIR)/myblob.js
	mv $(WASM_BUILD_DIR)/$(PROJECT_NAME)_bg.wasm $(WASM_BUILD_DIR)/myblob_bg.wasm

