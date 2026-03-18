use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=spacetimedb/src/lib.rs");
    println!("cargo:rerun-if-env-changed=SPACETIME_CLI");

    let generated_dir = PathBuf::from("src").join("module_bindings").join("generated");
    let generated_mod = generated_dir.join("mod.rs");
    let module_dir = PathBuf::from("spacetimedb");

    if try_generate_bindings(&module_dir, &generated_dir).is_err() && !generated_mod.exists() {
        fs::create_dir_all(&generated_dir).expect("create generated bindings dir");
        write_placeholder_bindings(&generated_mod).expect("write placeholder module bindings");
        println!("cargo:warning=SpacetimeDB CLI generation unavailable; using placeholder generated bindings.");
    }
}

fn try_generate_bindings(_module_dir: &Path, out_dir: &Path) -> std::io::Result<()> {
    let cli = env::var("SPACETIME_CLI").unwrap_or_else(|_| {
        let local = PathBuf::from(
            env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from(r"C:\Users\Default\AppData\Local")),
        )
        .join("SpacetimeDB")
        .join("spacetime.exe");

        if local.exists() {
            local.to_string_lossy().into_owned()
        } else {
            "spacetime".to_string()
        }
    });
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let wasm_path = PathBuf::from("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("autoloop_spacetimedb_module.wasm");

    let build_status = Command::new(cargo)
        .args([
            "build",
            "--manifest-path",
            "spacetimedb/Cargo.toml",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
        ])
        .status()?;

    if !build_status.success() {
        return Err(std::io::Error::other("cargo build for spacetimedb module failed"));
    }

    fs::create_dir_all(out_dir)?;
    let status = Command::new(cli)
        .args(["generate", "--lang", "rust", "--bin-path"])
        .arg(wasm_path)
        .args(["--out-dir"])
        .arg(out_dir)
        .args(["--yes"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(
            "spacetime generate exited with a non-zero status",
        ))
    }
}

fn write_placeholder_bindings(target: &Path) -> std::io::Result<()> {
    fs::write(
        target,
        r#"pub const GENERATED_WITH_SPACETIME_CLI: bool = false;

pub fn generation_hint() -> &'static str {
    "Install the SpacetimeDB CLI and rebuild to generate strongly typed module bindings."
}
"#,
    )
}
