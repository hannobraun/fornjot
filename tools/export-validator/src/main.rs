mod ffi;

use std::{ffi::CString, fs, process::Command, ptr};

use anyhow::{anyhow, bail, Context as _};

fn main() -> anyhow::Result<()> {
    for model in fs::read_dir("models")? {
        let model = model?;
        let model = model.file_name().into_string().map_err(|err| {
            anyhow!("Failed to convert directory name to `String`: {:?}", err)
        })?;

        let export_file = format!("{model}.3mf");

        let exit_status = Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(["--model", &model])
            .args(["--export", &export_file])
            .status()?;

        if !exit_status.success() {
            bail!(
                "Exporting model `{model}` failed with error code:\
                {exit_status}"
            );
        }

        // Presumably we're using the library in the way it's intended, so this
        // might be sound?
        unsafe {
            validate_model(&export_file).with_context(|| {
                format!("Could not validate model `{model}`")
            })?;
        }
    }

    Ok(())
}

unsafe fn validate_model(file: &str) -> anyhow::Result<()> {
    let mut model = ptr::null();

    let result = ffi::lib3mf_createmodel(&mut model);
    if result != 0 {
        bail!("Failed to create model; error code: {result}");
    }

    let mut reader = ptr::null();
    let reader_class = CString::new("3mf")?;

    let result = ffi::lib3mf_model_queryreader(
        model,
        reader_class.as_ptr(),
        &mut reader,
    );
    if result != 0 {
        bail!("Failed to query reader; error code: {result}");
    }

    let result = ffi::lib3mf_reader_setstrictmodeactive(reader, true);
    if result != 0 {
        bail!("Failed to set strict mode; error code: {result}");
    }

    let path = CString::new(file)?;

    let result = ffi::lib3mf_reader_readfromfile(reader, path.as_ptr());
    if result != 0 {
        bail!("Failed to read model; error code: {result}");
    }

    let mut num_warnings = 0;

    let result = ffi::lib3mf_reader_getwarningcount(reader, &mut num_warnings);
    if result != 0 {
        bail!("Failed to get number of warnings; error code: {result}");
    }

    if num_warnings > 0 {
        bail!(
            "Warnings while reading model; number of warnings: {num_warnings}"
        );
    }

    let mut object_iterator = ptr::null();

    let result = ffi::lib3mf_model_getobjects(model, &mut object_iterator);
    if result != 0 {
        bail!("Failed to get object iterator; error code: {result}");
    }

    loop {
        let mut has_next = false;

        let result = ffi::lib3mf_resourceiterator_movenext(
            object_iterator,
            &mut has_next,
        );
        if result != 0 {
            bail!(
                "Failed to move iterator to next object; error code: {result}"
            );
        }

        if !has_next {
            break;
        }

        let mut object = ptr::null();

        let result = ffi::lib3mf_objectiterator_getcurrentobject(
            object_iterator,
            &mut object,
        );
        if result != 0 {
            bail!("Failed to get object; error code: {result}");
        }

        let mut is_valid = false;

        let result = ffi::lib3mf_object_isvalid(object, &mut is_valid);
        if result != 0 {
            bail!(
                "Failed to determine if object is valid; error code: {result}"
            );
        }

        if !is_valid {
            // Yes, this error message is a bit sparse. If anyone is interested
            // in expanding this program, you're welcome to do that.
            //
            // However, the point here is to fail the CI build if something is
            // wrong. Once you know *that* the file is wrong, you can use an
            // existing validator to get more information. This is probably more
            // productive than expending effort to turn this into the perfect
            // validator.
            bail!("Object is not valid");
        }
    }

    Ok(())
}
