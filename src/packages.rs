use std::{fs::read_dir, path::Path};

const PACMAN_DIR: &str = "/var/lib/pacman/local";

pub fn get_num_packages() -> i16 {
    match read_dir(Path::new(PACMAN_DIR)) {
        Ok(entries) => entries.count() as i16,
        Err(_) => return 0
    }
}
