use std::{env, fs, path::PathBuf};

const SCHEMA_DIR: &str = "spec";
const ROOT_SCHEMA: &str = "core.fbs";
const OUT_FILE: &str = "flat.rs";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={SCHEMA_DIR}");

    let schema_path = PathBuf::from(SCHEMA_DIR).join(ROOT_SCHEMA);
    let declarations = planus_translation::translate_files(&[schema_path.as_path()]).unwrap();
    let raw_out = planus_codegen::generate_rust(&declarations, false)
        .unwrap()
        .replace("#[no_implicit_prelude]\n", "");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(OUT_FILE);
    fs::write(out_path, raw_out.as_bytes()).unwrap();
}
