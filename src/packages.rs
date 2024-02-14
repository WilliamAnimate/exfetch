use std::{fs::read_dir, path::Path};

const PACMAN_DIR: &str = "/var/lib/pacman/local";

pub fn get_num_packages() -> i16 {
    match read_dir(Path::new(PACMAN_DIR)) {
        Ok(entries) => entries.count() as i16 - 1, // -1 because there is an ALPM_DB_VERSION file at that dir
        Err(_) => return 0
    }
}
