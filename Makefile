.DEFAULT_GOAL := build

install_targets:
	#################### Installing Cross ########################
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu
.PHONY:install_targets

build: install_targets
	#################### Windows Build ########################
	cargo build --target x86_64-pc-windows-gnu --release
	#TARGET_CC=x86_64-unknown-linux-gnu cargo build --target x86_64-unknown-linux-gnu --release
.PHONY:build
