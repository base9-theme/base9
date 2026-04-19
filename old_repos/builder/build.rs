use std::{env};
use std::path::Path;
use std::io::{BufWriter};
use std::fs::File;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/default_config.yml");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("default_config.json");
    let src_path = "src/default_config.yml";
    let yml = std::fs::File::open(Path::new(src_path)).unwrap();
    let config: serde_json::Value = serde_yaml::from_reader(yml).unwrap();

    println!("OUT: {}", out_dir);
    println!("PATH: {}", dest_path.display());
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    serde_json::to_writer(f, &config);
}
