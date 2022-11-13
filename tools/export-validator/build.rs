use anyhow::bail;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
    if cfg!(target_family = "windows") {
        let out_dir = env::var("OUT_DIR")?;
        // TODO: Refactor once possible: https://github.com/rust-lang/cargo/issues/9661
        let Some(bin_dir) = Path::new(&out_dir).ancestors().nth(3) else {
            bail!("Failed to get bin_dir from this out_dir: {out_dir}");
        };
        // Note: Other rpath alternatives: https://ibob.bg/blog/2018/12/16/windows-rpath/
        fs::copy(libs_dir.join("lib3mf.dll"), bin_dir.join("lib3mf.dll"))?;
    }

    Ok(())
}
