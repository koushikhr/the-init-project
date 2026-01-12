use crate::PackageManager;
// Import all available managers
use crate::managers::flatpak::Flatpak;
use crate::managers::pacman::Pacman;
use crate::managers::paru::Paru;
use crate::managers::winget::Winget;

pub fn detect_managers() -> Vec<Box<dyn PackageManager>> {
    let info = os_info::get();
    println!("{}", info.to_string());

    let mut managers: Vec<Box<dyn PackageManager>> = Vec::new();

    match info.os_type() {
        os_info::Type::Windows => {
            managers.push(Box::new(Winget));
        }

        // If we are on Arch Linux, Manjaro, Garuda etc... use pacman
        os_info::Type::Arch | os_info::Type::Manjaro | os_info::Type::CachyOS => {
            // CHECK: Is Paru available?
            if is_binary_available("paru") {
                println!("Detected Paru. Using it as Primary Package Manager.");
                managers.push(Box::new(Paru));
            } else {
                println!("No AUR Helper found. Falling back to Pacman");
                managers.push(Box::new(Pacman));
            }
        }

        _ => {
            println!("Warning: OS not explicitly supported, defaulting to Pacman.");
            managers.push(Box::new(Pacman));
        }
    }

    // Universal Check
    if is_binary_available("flatpak") {
        println!("Detected Flatpak.");
        managers.push(Box::new(Flatpak));
    }

    managers
}

fn is_binary_available(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
