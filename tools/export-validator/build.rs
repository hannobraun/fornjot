use std::env;

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let libs_dir_relative = "lib3mf/libs";
    let libs_dir = format!("{manifest_dir}/{libs_dir_relative}");

    // This is necessary to link against the dynamic library.
    println!("cargo:rustc-link-search=native={libs_dir}");
    println!("cargo:rustc-link-lib=dylib=3mf");

    // And this is necessary, so the linked library is found at runtime.
    println!("cargo:rustc-link-arg=-Wl,-rpath,{libs_dir}");

    Ok(())
}
