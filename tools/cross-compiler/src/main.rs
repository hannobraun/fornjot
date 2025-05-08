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

        // This is part of a workaround to make `getrandom` build when compiling
        // for WASM. See comment in `fj-viewer`'s `Cargo.toml` for more
        // information.
        //
        // Setting the flag like this might not be appropriate, as it might
        // require a similar workaround from users of `fj-viewer`. However, as
        // of this writing (2025-05-08), `fj-viewer doesn't support the WASM
        // target in any meaningful way. It is only compiled to that target
        // during the CI build, to prevent regressions that would inhibit adding
        // support harder from slipping in.
        //
        // So given this context, the following workaround is probably fine.
        // Once real support for `fj-viewer` in WASM environments is added, we
        // probably have to come up with a different solution. (And then, we'll
        // be in a better position to actually test these different solutions.)
        if target.triple == "wasm32-unknown-unknown" {
            rust_flags.push_str(" --cfg getrandom_backend=\"wasm_js\"");
        }

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
