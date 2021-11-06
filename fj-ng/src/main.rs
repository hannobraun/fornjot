use std::process::Command;

fn main() -> anyhow::Result<()> {
    // This can be made a bit more contact using `ExitStatus::exit_ok`, once
    // that is stable.
    let status = Command::new("cargo")
        .arg("build")
        .args(["--manifest-path", "model/Cargo.toml"])
        .status()?;
    assert!(status.success());

    // TASK: Read up why those calls are unsafe. Make sure calling them is
    //       sound, and document why that is.
    let _model = unsafe {
        let lib = libloading::Library::new("model/target/debug/libmodel.so")?;
        let func: libloading::Symbol<ModelFn> = lib.get(b"model")?;
        func()
    };

    Ok(())
}

type ModelFn = unsafe extern "C" fn() -> fj::Model;
