fn main() -> anyhow::Result<()> {
    unsafe {
        let lib =
            libloading::Library::new("../model/target/debug/libmodel.so")?;
        let func: libloading::Symbol<unsafe extern "C" fn()> =
            lib.get(b"model")?;
        func()
    }

    Ok(())
}
