// Made by William Animate [https://github.com/WilliamAnimate]
use std::{fs, path::Path};

const PACMAN_DIR: &str = "/var/lib/pacman/local";
const DNF_DIR: &str = "/var/cache/dnf";
const APT_DIR: &str = "/var/cache/apt/archives";

macro_rules! count_files_in_directory {
    ($path:expr) => {
        fs::read_dir($path)
            .map(|dir| dir.count() as i16)
            .ok()
    }
}

macro_rules! file_exists_in_bin {
    ($file_to_search:expr) => {
        Path::new(&format!("/bin/{}", $file_to_search)).exists()
    }
}

pub enum PackageManager {
    Pacman,
    Dnf,
    Apt,
    Unknown,
}

pub fn get_num_packages() -> Option<i16> {
    let package_manager = detect_package_manager();
    match package_manager {
        PackageManager::Pacman => count_files_in_directory!(PACMAN_DIR),
        PackageManager::Dnf => count_files_in_directory!(DNF_DIR),
        PackageManager::Apt => count_files_in_directory!(APT_DIR),
        PackageManager::Unknown => {
            todo!("handle errors here, instead of returning None.\nBECAUSE THEN IT WILL CRASH.");
            // None
        }
    }
}

fn detect_package_manager() -> PackageManager {
    if file_exists_in_bin!("pacman") {
        PackageManager::Pacman
    } else if file_exists_in_bin!("dnf") {
        PackageManager::Dnf
    } else if file_exists_in_bin!("apt") {
        PackageManager::Apt
    } else {
        PackageManager::Unknown
    }
}
