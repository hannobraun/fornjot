use std::{env::current_dir, path::PathBuf};

use rfd::FileDialog;

pub fn show_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_directory(current_dir().unwrap_or_else(|_| PathBuf::from("/")))
        .pick_folder()
}
