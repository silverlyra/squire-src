#!/usr/bin/env just --justfile

build: sqlite
    cargo build

[working-directory: 'sqlite']
sqlite: prepare
    ../vendor/sqlite/configure
    make sqlite3.c

update version: prepare
    git submodule set-branch --branch tags/version-{{ version }} vendor/sqlite
    git submodule sync
    git submodule update --remote

prepare:
    [ -d vendor/sqlite ] || git submodule update --init
    mkdir -p sqlite

clean:
    rm -fr sqlite
    cargo clean
