// Represents platform trait
pub trait Platform {
    fn model_lib_file_name(&self, name: &str) -> String;
}

// Represents all supported platforms

// Mac OS
struct Macos;
// Windows
struct Windows;
// Linux
struct Unix;

impl Platform for Windows {
    fn model_lib_file_name(&self, name: &str) -> String {
        format!("{}.dll", name)
    }
}

impl Platform for Macos {
    fn model_lib_file_name(&self, name: &str) -> String {
        format!("lib{}.dylib", name)
    }
}

impl Platform for Unix {
    fn model_lib_file_name(&self, name: &str) -> String {
        format!("lib{}.so", name)
    }
}

// Abstracts over differences in host platforms
pub struct HostPlatform;

impl HostPlatform {
    pub fn get_os() -> Box<dyn Platform> {
        if cfg!(windows) {
            Box::new(Windows)
        } else if cfg!(target_os = "macos") {
            Box::new(Macos)
        } else {
            Box::new(Unix)
        }
    }

    pub fn lib_file_name(name: &str) -> String {
        Self::get_os().model_lib_file_name(name)
    }
}
