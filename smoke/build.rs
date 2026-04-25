use std::path::PathBuf;

fn main() {
    let config = sqlite::config(None);

    let dest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace dir")
        .join("build");
    let location = sqlite::Location::new(dest);

    sqlite::build(location, &config);
}
