#!/usr/bin/env just --justfile

prepare:
    mkdir -p amalgamation

[working-directory: 'amalgamation']
amalgamate: prepare
    ../sqlite/configure
    make sqlite3.c

clean:
    rm -fr amalgamation
    cargo clean
