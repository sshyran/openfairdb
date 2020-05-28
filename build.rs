use std::{env, path::Path, process::Command};

const APP_NAME: &str = "ofdb-app";
const APP_PKG_DIR: &str = "ofdb-app";
const APP_PKG_SRC: &str = "ofdb-app/src";

fn main() {
    if env::var("CARGO_FEATURE_DIAGNOSIS").is_ok() {
        assert_wasm_pack_is_installed();
        Command::new("wasm-pack")
            .args(&[
                "build",
                "--target",
                "web",
                "--release",
                "--out-name",
                APP_NAME,
            ])
            .current_dir(&Path::new(&APP_PKG_DIR))
            .status()
            .expect("Unable to successfully execute wasm-pack");
        println!("cargo:rerun-if-changed={}", APP_PKG_SRC);
        for entry in walkdir::WalkDir::new(APP_PKG_SRC)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}

fn assert_wasm_pack_is_installed() {
    let output = Command::new("cargo")
        .args(&["install", "--list"])
        .output()
        .expect("Unable to check wasm-pack installation");
    let output_string = String::from_utf8(output.stdout).unwrap();
    if !output_string.contains("wasm-pack") {
        Command::new("cargo")
            .args(&["install", "wasm-pack"])
            .status()
            .expect("Unable install wasm-pack");
    }
}
