//! Cross-compiler for the Fornjot build
//!
//! This tools cross-compiles the Fornjot crates that support that to the
//! targets they support. This is less resource-intense than using a `matrix` in
//! GitHub Actions (which would start one build job per crate and target), and
//! allows for the cross-compilation code to be re-used in `justfile`.

use std::process::Command;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let targets = [
        Target {
            triple: "aarch64-apple-ios",
            crates: &["fj-core", "fj-export", "fj-interop", "fj-math"],
        },
        Target {
            triple: "aarch64-linux-android",
            crates: &["fj-core", "fj-export", "fj-interop", "fj-math"],
        },
        Target {
            triple: "wasm32-unknown-unknown",
            crates: &[
                "fj-core",
                "fj-export",
                "fj-interop",
                "fj-math",
                "fj-viewer",
            ],
        },
    ];

    for target in targets {
        let mut rust_flags = String::new();
        rust_flags.push_str("-D warnings");

        for crate_ in target.crates {
            let mut command = Command::new("cargo");
            command
                .arg("build")
                .arg("--all-features")
                .args(["--target", target.triple])
                .args(["-p", crate_])
                .env("RUSTFLAGS", &rust_flags);

            println!("Running {command:?}");
            let status = command.status()?;

            if !status.success() {
                return Err(anyhow!("Cargo exited with error code: {status}"));
            }
        }
    }

    Ok(())
}

struct Target {
    /// The target triple
    triple: &'static str,

    /// The crates that are supported on this target
    crates: &'static [&'static str],
}
