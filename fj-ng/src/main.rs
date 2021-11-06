fn main() -> anyhow::Result<()> {
    unsafe {
        let lib = libloading::Library::new("model/target/debug/libmodel.so")?;
        let func: libloading::Symbol<ModelFn> = lib.get(b"model")?;
        func()
    }

    Ok(())
}

type ModelFn = unsafe extern "C" fn();
