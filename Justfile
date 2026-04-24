#!/usr/bin/env just --justfile

build: sqlite
    cargo build

check: sqlite
    cargo check -p squire-sqlite3-smoke

[working-directory('sqlite')]
sqlite: prepare
    ../vendor/sqlite/configure
    make sqlite3.c
    cp ../vendor/sqlite/README.md ../vendor/sqlite/LICENSE.md ../vendor/sqlite/VERSION .

release version: clean (update version) prepare check

update version: prepare
    git checkout -b 'release-v{{ version }}-alpha.1' main
    cargo set-version '{{ version }}-alpha.1'

    git submodule set-branch --branch tags/version-{{ version }} vendor/sqlite
    git submodule sync
    git submodule update --remote

    git add -u
    git commit -m 'Update to SQLite v{{ version }}'
    git push -u origin $(git branch --show-current)

tag version:
    git tag -a 'v{{ version }}-alpha.1' -m 'SQLite {{ version }}'

prepare:
    [ -d vendor/sqlite ] || git submodule update --init
    mkdir -p sqlite

clean:
    rm -fr sqlite
