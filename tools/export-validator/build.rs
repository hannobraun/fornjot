use std::{env, path::PathBuf};

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let mut libs_dir = PathBuf::from(manifest_dir);
    libs_dir.push("lib3mf");
    libs_dir.push("libs");

    // This is necessary to link against the dynamic library.
    println!("cargo:rustc-link-search=native={}", libs_dir.display());
    println!("cargo:rustc-link-lib=dylib=3mf");

    // And this is necessary, so the linked library is found at runtime.
    if cfg!(target_family = "unix") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
    }

    Ok(())
}
