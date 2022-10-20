//! Cross-compiler for the Fornjot build
//!
//! This tools cross-compiles the Fornjot crates that support that to the
//! targets they support. This is less resource-intense then using a `matrix` in
//! GitHub Actions (which would start one build job per crate and target), and
//! allows for the cross-compilation code to be re-used in `justfile`.

use std::process::Command;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let crates = [
        "fj",
        "fj-export",
        "fj-interop",
        "fj-kernel",
        "fj-math",
        "fj-operations",
        "fj-proc",
        "fj-viewer",
    ];

    for crate_ in crates {
        let mut command = Command::new("cargo");
        command
            .arg("build")
            .arg("--all-features")
            .args(["--target", "wasm32-unknown-unknown"])
            .args(["-p", crate_])
            .env("RUSTFLAGS", "-D warnings");

        println!("Running {command:?}");
        let status = command.status()?;

        if !status.success() {
            return Err(anyhow!("Cargo exited with error code: {status}"));
        }
    }

    Ok(())
}
