use std::{
    env, fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

const SCHEMA_DIR: &str = "spec";
const ROOT_SCHEMA: &str = "core.fbs";
const OUT_FILE: &str = "flat.rs";

fn format_string(s: &str) -> Result<String, String> {
    let mut child = Command::new("rustfmt");

    child
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child
        .spawn()
        .map_err(|err| format!("Unable to spawn rustfmt. Perhaps it is not installed? {err}"))?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(s.as_bytes())
        .map_err(|err| format!("Unable to write generated code to rustfmt: {err}"))?;

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Unable to get formatted code back from rustfmt: {err}"))?;

    if output.status.success() && output.stderr.is_empty() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else if output.stderr.is_empty() {
        Err(format!("rustfmt failed with exit code {}", output.status))
    } else {
        Err(format!(
            "rustfmt failed with exit code {} and message:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={SCHEMA_DIR}");

    let schema_path = PathBuf::from(SCHEMA_DIR).join(ROOT_SCHEMA);
    let declarations = planus_translation::translate_files(&[schema_path.as_path()]).unwrap();
    let raw_out = planus_codegen::generate_rust(&declarations, false)
        .unwrap()
        .replace("#[no_implicit_prelude]\n", "");
    let formatted_out = format_string(&raw_out).unwrap();

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(OUT_FILE);
    fs::write(out_path, formatted_out.as_bytes()).unwrap();
}
