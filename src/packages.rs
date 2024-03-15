use std::fs::read_dir;
use std::path::Path;

const PACMAN_DIR: &str = "/var/lib/pacman/local";

#[must_use]
pub fn get_num_packages() -> i16 {
    read_dir(Path::new(PACMAN_DIR)).map_or(0, |entries| entries.count() as i16 - 1)
}
