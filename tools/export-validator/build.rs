use std::env;

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let libs_dir_relative = "lib3mf/libs";
    let libs_dir = format!("{manifest_dir}/{libs_dir_relative}");

    // This is necessary to link against the dynamic library.
    println!("cargo:rustc-link-search=native={libs_dir}");
    println!("cargo:rustc-link-lib=dylib=3mf");

    // And this is necessary, so the linked library is found at runtime.
    //
    // The relative path used here is designed to work when the validator is
    // executed within the repository using `cargo run`, which is the intended
    // use case.
    let executable_to_libs =
        format!("../../tools/export-validator/{libs_dir_relative}");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/{executable_to_libs}");

    Ok(())
}
