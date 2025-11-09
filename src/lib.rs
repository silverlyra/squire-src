//! # squire-sqlite3-src
//!
//! This crate bundles the [SQLite][] source code for [Squire][]. When Squire’s
//! `bundled` feature flag is enabled, SQLite is built from the `sqlite3.c` file
//! included in this crate, and linked into the [`squire-sqlite3-sys`][sys]
//! crate.
//!
//! Users of Squire don’t need to be aware of or interact with this crate. Cargo
//! will automatically download and build it when you enable Squire’s `bundled`
//! feature.
//!
//! [SQLite]: https://sqlite.org/
//! [Squire]: https://github.com/silverlyra/squire
//! [sys]: https://lib.rs/squire-sqlite3-sys

use std::{collections::HashMap, env, iter, ops::Deref, path::PathBuf};

use strum::{EnumDiscriminants, IntoDiscriminant};

/// Build the bundled SQLite sources, using the given [`Config`].
pub fn build(location: Location, config: impl AsRef<Config>) -> Build {
    let config = config.as_ref();

    let mut compiler = cc::Build::new();
    compiler.file(location.input());

    config.apply(&mut compiler);
    compiler.warnings(false);

    compiler.out_dir(&location.dest);
    compiler.compile("sqlite3");

    Build::new(location)
}

/// The output of [`Build`], including the [`Location`] SQLite was built into.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Build {
    location: Location,
}

impl Build {
    const fn new(location: Location) -> Build {
        Self { location }
    }

    /// The `.c` source files that need to be built (`sqlite3.c`).
    pub fn sources(&self) -> impl Iterator<Item = PathBuf> {
        iter::once(self.input())
    }

    /// The build [`Location`].
    pub const fn location(&self) -> &Location {
        &self.location
    }
}

impl Deref for Build {
    type Target = Location;

    fn deref(&self) -> &Self::Target {
        self.location()
    }
}

/// Specifies the source and target directories for [`build`].
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Location {
    src: PathBuf,
    dest: PathBuf,
}

impl Location {
    /// Create a build [`Location`] from `$CARGO_MANIFEST_DIR`.
    pub fn new(dest: impl Into<PathBuf>) -> Self {
        Self {
            src: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sqlite"),
            dest: dest.into(),
        }
    }

    /// The path to `sqlite3.c`.
    pub fn input(&self) -> PathBuf {
        self.src.join("sqlite3.c")
    }

    /// The path to `sqlite3.h`.
    pub fn header(&self) -> PathBuf {
        self.src.join("sqlite3.h")
    }

    /// The build's target directory.
    pub fn dest(&self) -> PathBuf {
        self.dest.clone()
    }

    /// Iterates source files (`sqlite3.c` and `sqlite3.h`).
    pub fn sources(&self) -> impl Iterator<Item = PathBuf> {
        iter::once(self.input()).chain(iter::once(self.header()))
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new(PathBuf::from(
            env::var_os("OUT_DIR").expect("$OUT_DIR not set"),
        ))
    }
}

/// Configures a SQLite [`build`], applying various [compile-time options][].
///
/// [compile-time options]: https://sqlite.org/compile.html
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Config {
    settings: HashMap<SettingKey, Setting>,
}

impl Config {
    /// Configure a build using the given [settings](Setting).
    pub fn new(settings: impl IntoIterator<Item = Setting>) -> Self {
        Self {
            settings: settings
                .into_iter()
                .map(|setting| (setting.discriminant(), setting))
                .collect(),
        }
    }

    /// Look up a [setting](Setting) by its [key](SettingKey).
    pub fn get(&self, key: SettingKey) -> Option<Setting> {
        self.settings.get(&key).copied()
    }

    /// Change a build [setting](Setting).
    pub fn set(&mut self, setting: Setting) {
        self.settings.insert(setting.discriminant(), setting);
    }

    fn apply(&self, build: &mut cc::Build) {
        for setting in self.settings.values() {
            setting.apply(build);
        }
    }
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new([
            Setting::Sync(Synchronous::Full),
            Setting::WalSync(Synchronous::Normal),
            Setting::Threading(Threading::MultiThread),
            Setting::DoubleQuotedStrings(DoubleQuotedStrings::default()),
            Setting::DefaultForeignKeys(true),
            Setting::DefaultMemoryStatus(false),
            Setting::EnableAlloca(true),
            Setting::EnableAuthorization(false),
            Setting::EnableAutomaticIndex(true),
            Setting::EnableAutomaticInitialize(true), // TODO
            Setting::EnableAutomaticReset(false),
            Setting::EnableBlobIo(false),
            Setting::EnableColumnDeclaredType(false),
            Setting::EnableDatabasePagesVirtualTable(false),
            Setting::EnableDatabaseStatisticsVirtualTable(false),
            Setting::EnableDatabaseUri(true),
            Setting::EnableDeprecated(false),
            Setting::EnableGetTable(false),
            Setting::EnableMemoryManagement(true),
            Setting::EnableProgressCallback(false),
            Setting::EnableSharedCache(false),
            Setting::EnableTrace(false),
            Setting::EnableUtf16(false),
            Setting::EnableVirtualTables(true),
            Setting::EnableWriteAheadLog(true),
            Setting::LikeOperatorMatchesBlob(false),
            Setting::MaxExpressionDepth(0),
            #[cfg(debug_assertions)]
            Setting::EnableApiArmor(true),
            #[cfg(debug_assertions)]
            Setting::Debug(true),
        ])
    }
}

/// A [compile-time option][] for a SQLite [`build`].
///
/// [compile-time option]: https://sqlite.org/compile.html
#[derive(EnumDiscriminants, PartialEq, Eq, Clone, Copy, Debug)]
#[strum_discriminants(name(SettingKey))]
#[strum_discriminants(derive(Hash))]
pub enum Setting {
    #[doc(alias = "SQLITE_DQS")]
    DoubleQuotedStrings(DoubleQuotedStrings),
    #[doc(alias = "SQLITE_THREADSAFE")]
    Threading(Threading),
    #[doc(alias = "SQLITE_DEBUG")]
    Debug(bool),
    #[doc(alias = "SQLITE_DEFAULT_SYNCHRONOUS")]
    Sync(Synchronous),
    #[doc(alias = "SQLITE_DEFAULT_WAL_SYNCHRONOUS")]
    WalSync(Synchronous),
    #[doc(alias = "SQLITE_DEFAULT_AUTOMATIC_INDEX")]
    DefaultAutomaticIndex(bool),
    #[doc(alias = "SQLITE_DEFAULT_AUTOVACUUM")]
    DefaultAutomaticVacuum(bool),
    #[doc(alias = "SQLITE_DEFAULT_FOREIGN_KEYS")]
    DefaultForeignKeys(bool),
    #[doc(alias = "SQLITE_DEFAULT_MEMSTATUS")]
    DefaultMemoryStatus(bool),
    #[doc(alias = "SQLITE_USE_ALLOCA")]
    EnableAlloca(bool),
    #[doc(alias = "SQLITE_ENABLE_API_ARMOR")]
    EnableApiArmor(bool),
    #[doc(alias = "SQLITE_OMIT_AUTHORIZATION")]
    EnableAuthorization(bool),
    #[doc(alias = "SQLITE_OMIT_AUTOMATIC_INDEX")]
    EnableAutomaticIndex(bool),
    #[doc(alias = "SQLITE_OMIT_AUTOINIT")]
    EnableAutomaticInitialize(bool),
    #[doc(alias = "SQLITE_OMIT_AUTORESET")]
    EnableAutomaticReset(bool),
    #[doc(alias = "SQLITE_OMIT_INCRBLOB")]
    EnableBlobIo(bool),
    #[doc(alias = "SQLITE_OMIT_DECLTYPE")]
    EnableColumnDeclaredType(bool),
    #[doc(alias = "SQLITE_ENABLE_COLUMN_METADATA")]
    EnableColumnMetadata(bool),
    #[doc(alias = "SQLITE_ENABLE_DBPAGE_VTAB")]
    EnableDatabasePagesVirtualTable(bool),
    #[doc(alias = "SQLITE_ENABLE_DBSTAT_VTAB")]
    EnableDatabaseStatisticsVirtualTable(bool),
    #[doc(alias = "SQLITE_USE_URI")]
    EnableDatabaseUri(bool),
    #[doc(alias = "SQLITE_OMIT_DEPRECATED")]
    EnableDeprecated(bool),
    #[doc(alias = "SQLITE_ENABLE_GEOPOLY")]
    EnableGeopoly(bool),
    #[doc(alias = "SQLITE_OMIT_GET_TABLE")]
    EnableGetTable(bool),
    #[doc(alias = "SQLITE_ENABLE_FTS3")]
    #[doc(alias = "SQLITE_ENABLE_FTS4")]
    EnableFts3(bool),
    #[doc(alias = "SQLITE_ENABLE_FTS5")]
    EnableFts5(bool),
    #[doc(alias = "SQLITE_ENABLE_JSON1")]
    #[doc(alias = "SQLITE_OMIT_JSON")]
    EnableJson(bool),
    #[doc(alias = "SQLITE_OMIT_LOAD_EXTENSION")]
    EnableLoadExtension(bool),
    #[doc(alias = "SQLITE_ENABLE_MEMORY_MANAGEMENT")]
    EnableMemoryManagement(bool),
    #[doc(alias = "SQLITE_ENABLE_NORMALIZE")]
    EnableNormalizeSql(bool),
    #[doc(alias = "SQLITE_ENABLE_PREUPDATE_HOOK")]
    EnablePreUpdateHook(bool),
    #[doc(alias = "SQLITE_OMIT_PROGRESS_CALLBACK")]
    EnableProgressCallback(bool),
    #[doc(alias = "SQLITE_ENABLE_RTREE")]
    EnableRtree(bool),
    #[doc(alias = "SQLITE_ENABLE_STAT4")]
    EnableStat4(bool),
    #[doc(alias = "SQLITE_OMIT_DESERIALIZE")]
    EnableSerialize(bool),
    #[doc(alias = "SQLITE_ENABLE_SESSION")]
    EnableSession(bool),
    #[doc(alias = "SQLITE_ENABLE_SNAPSHOT")]
    EnableSnapshot(bool),
    #[doc(alias = "SQLITE_OMIT_SHARED_CACHE")]
    EnableSharedCache(bool),
    #[doc(alias = "SQLITE_SOUNDEX")]
    EnableSoundex(bool),
    #[doc(alias = "SQLITE_OMIT_TCL_VARIABLE")]
    EnableTclVariables(bool),
    #[doc(alias = "SQLITE_OMIT_TEMPDB")]
    EnableTemporaryDatabase(bool),
    #[doc(alias = "SQLITE_OMIT_TRACE")]
    EnableTrace(bool),
    #[doc(alias = "SQLITE_OMIT_UTF16")]
    EnableUtf16(bool),
    #[doc(alias = "SQLITE_OMIT_VIRTUALTABLE")]
    EnableVirtualTables(bool),
    #[doc(alias = "SQLITE_OMIT_VIRTUALTABLE")]
    EnableWriteAheadLog(bool),
    #[doc(alias = "SQLITE_OMIT_WAL")]
    LikeOperatorCaseSensitive(bool),
    #[doc(alias = "SQLITE_LIKE_DOESNT_MATCH_BLOBS")]
    LikeOperatorMatchesBlob(bool),
    #[doc(alias = "SQLITE_MAX_ATTACHED")]
    MaxAttachedDatabases(usize),
    #[doc(alias = "SQLITE_MAX_COLUMN")]
    MaxColumns(usize),
    #[doc(alias = "SQLITE_MAX_EXPR_DEPTH")]
    MaxExpressionDepth(usize),
    #[doc(alias = "SQLITE_JSON_MAX_DEPTH")]
    MaxJsonDepth(usize),
    #[doc(alias = "SQLITE_MAX_VARIABLE_NUMBER")]
    MaxVariables(usize),
    #[doc(alias = "SQLITE_SECURE_DELETE")]
    SecureDelete(bool),
    #[doc(alias = "SQLITE_TEMP_STORE")]
    TemporaryStorage(TemporaryStorage),
    #[doc(alias = "SQLITE_TRUSTED_SCHEMA")]
    TrustedSchema(bool),
}

impl Setting {
    fn apply(&self, build: &mut cc::Build) {
        match *self {
            Setting::Debug(enable) => {
                self.define(build, "SQLITE_DEBUG", enable);
            }
            Setting::DefaultAutomaticIndex(enable) => {
                self.set(build, "SQLITE_DEFAULT_AUTOMATIC_INDEX", enable);
            }
            Setting::DefaultAutomaticVacuum(enable) => {
                self.set(build, "SQLITE_DEFAULT_AUTOVACUUM", enable);
            }
            Setting::DefaultForeignKeys(enable) => {
                self.set(build, "SQLITE_DEFAULT_FOREIGN_KEYS", enable);
            }
            Setting::DefaultMemoryStatus(enable) => {
                self.set(build, "SQLITE_DEFAULT_MEMSTATUS", enable);
            }
            Setting::DoubleQuotedStrings(DoubleQuotedStrings { in_ddl, in_dml }) => {
                let value = match (in_ddl, in_dml) {
                    (true, true) => 3,
                    (true, false) => 2,
                    (false, true) => 1,
                    (false, false) => 0,
                };

                self.set(build, "SQLITE_DQS", value);
            }
            Setting::EnableAlloca(enable) => {
                self.define(build, "SQLITE_USE_ALLOCA", enable);
            }
            Setting::EnableApiArmor(enable) => {
                self.define(build, "SQLITE_ENABLE_API_ARMOR", enable);
            }
            Setting::EnableAuthorization(enable) => {
                self.define(build, "SQLITE_OMIT_AUTHORIZATION", !enable);
            }
            Setting::EnableAutomaticIndex(enable) => {
                self.define(build, "SQLITE_OMIT_AUTOMATIC_INDEX", !enable);
            }
            Setting::EnableAutomaticInitialize(enable) => {
                self.define(build, "SQLITE_OMIT_AUTOINIT", !enable);
            }
            Setting::EnableAutomaticReset(enable) => {
                self.define(build, "SQLITE_OMIT_AUTORESET", !enable);
            }
            Setting::EnableBlobIo(enable) => {
                self.define(build, "SQLITE_OMIT_INCRBLOB", !enable);
            }
            Setting::EnableColumnDeclaredType(enable) => {
                self.define(build, "SQLITE_OMIT_DECLTYPE", !enable);
            }
            Setting::EnableColumnMetadata(enable) => {
                self.define(build, "SQLITE_ENABLE_COLUMN_METADATA", enable);
            }
            Setting::EnableDatabasePagesVirtualTable(enable) => {
                self.define(build, "SQLITE_ENABLE_DBPAGE_VTAB", enable);
            }
            Setting::EnableDatabaseStatisticsVirtualTable(enable) => {
                self.define(build, "SQLITE_ENABLE_DBSTAT_VTAB", enable);
            }
            Setting::EnableDatabaseUri(enable) => {
                self.set(build, "SQLITE_USE_URI", enable);
            }
            Setting::EnableDeprecated(enable) => {
                self.define(build, "SQLITE_OMIT_DEPRECATED", !enable);
            }
            Setting::EnableFts3(enable) => {
                self.define(build, "SQLITE_ENABLE_FTS3", enable);
                self.define(build, "SQLITE_ENABLE_FTS3_PARENTHESIS", enable);
            }
            Setting::EnableFts5(enable) => {
                self.define(build, "SQLITE_ENABLE_FTS5", enable);
            }
            Setting::EnableGeopoly(enable) => {
                self.define(build, "SQLITE_ENABLE_GEOPOLY", enable);
            }
            Setting::EnableGetTable(enable) => {
                self.define(build, "SQLITE_OMIT_GET_TABLE", !enable);
            }
            Setting::EnableJson(enable) => {
                self.define(build, "SQLITE_OMIT_JSON", !enable);
            }
            Setting::EnableLoadExtension(enable) => {
                self.define(build, "SQLITE_OMIT_LOAD_EXTENSION", !enable);
            }
            Setting::EnableMemoryManagement(enable) => {
                self.define(build, "SQLITE_ENABLE_MEMORY_MANAGEMENT", enable);
            }
            Setting::EnableNormalizeSql(enable) => {
                self.define(build, "SQLITE_ENABLE_NORMALIZE", enable);
            }
            Setting::EnablePreUpdateHook(enable) => {
                self.define(build, "SQLITE_ENABLE_PREUPDATE_HOOK", enable);
            }
            Setting::EnableProgressCallback(enable) => {
                self.define(build, "SQLITE_OMIT_PROGRESS_CALLBACK", !enable);
            }
            Setting::EnableRtree(enable) => {
                self.define(build, "SQLITE_ENABLE_RTREE", enable);
            }
            Setting::EnableSerialize(enable) => {
                self.define(build, "SQLITE_OMIT_DESERIALIZE", !enable);
            }
            Setting::EnableSession(enable) => {
                self.define(build, "SQLITE_ENABLE_SESSION", enable);
            }
            Setting::EnableSharedCache(enable) => {
                self.define(build, "SQLITE_OMIT_SHARED_CACHE", !enable);
            }
            Setting::EnableSnapshot(enable) => {
                self.define(build, "SQLITE_ENABLE_SNAPSHOT", enable);
            }
            Setting::EnableSoundex(enable) => {
                self.define(build, "SQLITE_SOUNDEX", enable);
            }
            Setting::EnableStat4(enable) => {
                self.define(build, "SQLITE_ENABLE_STAT4", enable);
            }
            Setting::EnableTclVariables(enable) => {
                self.define(build, "SQLITE_OMIT_TCL_VARIABLE", !enable);
            }
            Setting::EnableTemporaryDatabase(enable) => {
                self.define(build, "SQLITE_OMIT_TEMPDB", !enable);
            }
            Setting::EnableTrace(enable) => {
                self.define(build, "SQLITE_OMIT_TRACE", !enable);
            }
            Setting::EnableUtf16(enable) => {
                self.define(build, "SQLITE_OMIT_UTF16", !enable);
            }
            Setting::EnableVirtualTables(enable) => {
                self.define(build, "SQLITE_OMIT_VIRTUALTABLE", !enable);
            }
            Setting::EnableWriteAheadLog(enable) => {
                self.define(build, "SQLITE_OMIT_WAL", !enable);
            }
            Setting::LikeOperatorCaseSensitive(enable) => {
                self.define(build, "SQLITE_CASE_SENSITIVE_LIKE", enable);
            }
            Setting::LikeOperatorMatchesBlob(enable) => {
                self.define(build, "SQLITE_LIKE_DOESNT_MATCH_BLOBS", !enable);
            }
            Setting::MaxAttachedDatabases(max) => {
                self.set(build, "SQLITE_MAX_ATTACHED", max);
            }
            Setting::MaxColumns(max) => {
                self.set(build, "SQLITE_MAX_COLUMN", max);
            }
            Setting::MaxExpressionDepth(max) => {
                self.set(build, "SQLITE_MAX_EXPR_DEPTH", max);
            }
            Setting::MaxJsonDepth(max) => {
                self.set(build, "SQLITE_JSON_MAX_DEPTH", max);
            }
            Setting::MaxVariables(max) => {
                self.set(build, "SQLITE_MAX_VARIABLE_NUMBER", max);
            }
            Setting::SecureDelete(enable) => {
                self.define(build, "SQLITE_SECURE_DELETE", enable);
            }
            Setting::Sync(synchronous) => {
                self.set(build, "SQLITE_DEFAULT_SYNCHRONOUS", synchronous);
            }
            Setting::Threading(threading) => {
                self.set(build, "SQLITE_THREADSAFE", threading);
            }
            Setting::TemporaryStorage(mode) => {
                self.set(build, "SQLITE_TEMP_STORE", mode);
            }
            Setting::TrustedSchema(enable) => {
                self.set(build, "SQLITE_TRUSTED_SCHEMA", enable);
            }
            Setting::WalSync(synchronous) => {
                self.set(build, "SQLITE_DEFAULT_WAL_SYNCHRONOUS", synchronous);
            }
        }
    }

    fn define(&self, build: &mut cc::Build, name: &'static str, enable: bool) {
        if enable {
            build.define(name, None);
        }
    }

    fn set(&self, build: &mut cc::Build, name: &'static str, value: impl SettingValue) {
        value.apply(build, name);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct DoubleQuotedStrings {
    in_ddl: bool,
    in_dml: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum TemporaryStorage {
    AlwaysFilesystem = 0,
    DefaultFilesystem = 1,
    DefaultMemory = 2,
    AlwaysMemory = 3,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum Threading {
    SingleThread = 0,
    MultiThread = 1,
    Serialized = 2,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum Synchronous {
    Off = 0,
    Normal = 1,
    Full = 2,
    Extra = 3,
}

trait SettingValue {
    fn apply(&self, build: &mut cc::Build, name: &'static str);
}

impl SettingValue for bool {
    fn apply(&self, build: &mut cc::Build, name: &'static str) {
        let value = if *self { "1" } else { "0" };
        build.define(name, value);
    }
}

impl SettingValue for usize {
    fn apply(&self, build: &mut cc::Build, name: &'static str) {
        let value = self.to_string();
        build.define(name, value.as_str());
    }
}

impl SettingValue for TemporaryStorage {
    fn apply(&self, build: &mut cc::Build, name: &'static str) {
        (*self as usize).apply(build, name);
    }
}

impl SettingValue for Threading {
    fn apply(&self, build: &mut cc::Build, name: &'static str) {
        (*self as usize).apply(build, name);
    }
}

impl SettingValue for Synchronous {
    fn apply(&self, build: &mut cc::Build, name: &'static str) {
        (*self as usize).apply(build, name);
    }
}
