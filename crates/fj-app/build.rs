use std::process::{Command, Output, Stdio};

fn main() {
    println!("cargo:rustc-env=FJ_VERSION_STRING={}", version_string());
}

fn version_string() -> String {
    let pkg_version = std::env::var("CARGO_PKG_VERSION")
        .expect("The $CARGO_PKG_VERSION variable wasn't set");
    let commit = git_description();

    let official_release = std::env::var("FJ_OFFICIAL_RELEASE").is_ok();
    println!("cargo:rerun-if-env-changed=FJ_OFFICIAL_RELEASE");

    match (commit, official_release) {
        (Some(commit), true) => format!("{pkg_version} ({commit})"),
        (Some(commit), false) => {
            format!("{pkg_version} ({commit}, unreleased)")
        }
        (None, true) => pkg_version,
        (None, false) => format!("{pkg_version} (unreleased)"),
    }
}

fn git_description() -> Option<String> {
    // Note: it's okay for this to fail to start if git isn't installed (e.g.
    // because we're building in a docker container), but any errors returned by
    // git itself should fail the build.

    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(["describe", "--always", "--dirty=-modified"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&stdout);

    if !status.success() {
        let stderr = String::from_utf8_lossy(&stderr);
        eprintln!("---- Stdout ----");
        eprintln!("{stdout}");
        eprintln!("---- Stderr ----");
        eprintln!("{stderr}");
        panic!("Git exited unsuccessfully");
    }

    Some(stdout.trim().to_string())
}
