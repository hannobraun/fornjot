// Represents platform trait
trait Platform {
    fn file_name(name: &str) -> String;
}

// Represents all platforms supported

// Mac OS
struct Macos;
// Windows
struct Windows;
// Linux
struct Unix;

impl Platform for Windows {
    fn file_name(name: &str) -> String {
        format!("{}.dll", name)
    }
}
impl Platform for Macos {
    fn file_name(name: &str) -> String {
        format!("lib{}.dylib", name)
    }
}
impl Platform for Unix {
    fn file_name(name: &str) -> String {
        format!("lib{}.so", name)
    }
}

// Represents common apis availiable independent of hosts
pub struct HostPlatform;

impl HostPlatform {
    pub fn host_file_name(name: &str) -> String {
        if cfg!(windows) {
            Windows::file_name(name)
        } else if cfg!(target_os = "macos") {
            Macos::file_name(name)
        } else {
            //Unix
            Unix::file_name(name)
        }
    }
}
