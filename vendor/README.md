## `vendor`

Within `vendor`, a Git [submodule][] is used to include the source of [SQLite][]
so that [Squire][]’s `bundled` feature can be used to build a fresh copy of
SQLite.

SQLite has been placed [in the public domain][copyright].

### Fetching the SQLite source

If there is no `sqlite` directory present here, run:

```sh
git submodule update --init
```

[submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules
[SQLite]: https://sqlite.org/
[Squire]: https://github.com/silverlyra/squire
[copyright]: https://sqlite.org/copyright.html
