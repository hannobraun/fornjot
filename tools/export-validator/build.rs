use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let mut libs_dir = PathBuf::from(manifest_dir);
    libs_dir.push("lib3mf");
    libs_dir.push("libs");

    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    // This is necessary to link against the dynamic library.
    println!("cargo:rustc-link-search=native={}", libs_dir.display());
    println!("cargo:rustc-link-lib=dylib=3mf");

    // And this is necessary, so the linked library is found at runtime.
    if cfg!(target_family = "unix") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", libs_dir.display());
    }
    if cfg!(target_family = "windows") {
        fs::copy(
            libs_dir.join("3mf.dll"),
            out_dir.join("../../../deps/3mf.dll"),
        )?;
    }

    Ok(())
}
