use ignore::WalkBuilder;
use std::{collections::HashSet, env, ffi::OsStr, fs::File, path::Path};

static NEW_MODEL_TEMPLATE: &str = "star";
static EXTRA_IGNORED_FILES: &[&str] = &["star.png", "README.md"];

fn main() {
    create_new_model_tar();
}

fn create_new_model_tar() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file = File::create(Path::new(&out_dir).join("new_model.tar")).unwrap();
    let mut tar_builder = tar::Builder::new(file);

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let fornjot_root_path = Path::new(&manifest_dir)
        .ancestors()
        .nth(2)
        .expect("Failed to get 'fornjot_root_path' path");

    let new_model_path =
        fornjot_root_path.join("models").join(NEW_MODEL_TEMPLATE);

    let extra_ignored_files = EXTRA_IGNORED_FILES
        .iter()
        .map(OsStr::new)
        .collect::<HashSet<_>>();

    for entry in WalkBuilder::new(&new_model_path).hidden(false).build() {
        let path = entry.unwrap().into_path();
        if path.is_dir()
            || extra_ignored_files.contains(&path.file_name().unwrap())
        {
            continue;
        }
        let tar_path = path.strip_prefix(&new_model_path).unwrap();
        tar_builder
            .append_file(tar_path, &mut File::open(&path).unwrap())
            .unwrap();
    }
    tar_builder.finish().unwrap();
}
