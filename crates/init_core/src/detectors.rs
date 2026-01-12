use crate::PackageManager;
// Import all available managers
use crate::managers::pacman::Pacman;
use crate::managers::paru::Paru;
use crate::managers::winget::Winget;

pub fn get_system_manager() -> Box<dyn PackageManager> {
    let info = os_info::get();
    println!("{}", info.to_string());

    match info.os_type() {
        os_info::Type::Windows => Box::new(Winget),

        // If we are on Arch Linux, Manjaro, Garuda etc... use pacman
        os_info::Type::Arch | os_info::Type::Manjaro | os_info::Type::CachyOS => {
            // CHECK: Is Paru available?
            // We can't use async in this synchronous factory function easily,
            // so we use standard std::process for this quick check.

            if is_binary_available("paru") {
                println!("Detected Paru. Using it as Primary Package Manager.");
                Box::new(Paru)
            } else {
                println!("No AUR Helper found. Falling back to Pacman");
                Box::new(Pacman)
            }
        }

        _ => {
            println!("Warning: OS not explicitly supported, defaulting to Pacman.");
            Box::new(Pacman)
        }
    }
}

fn is_binary_available(name: &str) -> bool {
    std::process::Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
