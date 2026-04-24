use std::path::PathBuf;

use sqlite::{Config, Location, build};

fn main() {
    let config = Config::default();

    let dest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace dir")
        .join("build");
    let location = Location::new(dest);

    build(location, &config);
}
