INSTALL_PATH		:=$(HOME)/opt/libexec/
CARBON14_NAME		:=carbon14
CARBON14_VERSION	:=$(shell cargo run -- --version | awk '{ print $$NF }')
CARBON14_DEBUG_EXEC	:=target/debug/$(CARBON14_NAME)
CARBON14_RELEASE_EXEC	:=target/release/$(CARBON14_NAME)
CARBON14_EXEC		:=$(CARBON14_DEBUG_EXEC)
CARBON14_RUN		:=$(CARBON14_DEBUG_EXEC)
CARBON14_RUN		:=cargo run --bin $(CARBON14_NAME) --

all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(CARBON14_RELEASE_EXEC): $(INSTALL_PATH)
	cargo build --release

$(CARBON14_DEBUG_EXEC): $(INSTALL_PATH)
	cargo build

release: check fix | $(CARBON14_RELEASE_EXEC)
	install $(CARBON14_RELEASE_EXEC) $(INSTALL_PATH)/$(CARBON14_NAME)-$(CARBON14_VERSION)
	install $(CARBON14_RELEASE_EXEC) $(INSTALL_PATH)

debug: check fix | $(CARBON14_DEBUG_EXEC)
	install $(CARBON14_DEBUG_EXEC) $(INSTALL_PATH)/$(CARBON14_NAME)-$(CARBON14_VERSION)
	install $(CARBON14_DEBUG_EXEC) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cleanx:
	@rm -rf $(CARBON14_DEBUG_EXEC)
	@rm -rf $(CARBON14_RELEASE_EXEC)

cls:
	-@reset || tput reset

fix:
	cargo fix

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

test: build
	cargo run -- Cargo.toml README.md

build: check
	cargo $@

run: cleanx $(CARBON14_DEBUG_EXEC)
	$(CARBON14_RUN) -o target/debug/checksums.yaml target/debug/*
	$(CARBON14_RUN) -o target/checksums.yaml target


.PHONY: all clean cls release debug fix fmt check build test
