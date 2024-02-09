// Made by William Animate [https://github.com/WilliamAnimate]
use std::{fs, path::Path};

const PACMAN_DIR: &str = "/var/lib/pacman/local";

pub fn get_num_packages() -> Option<i16> {
    fs::read_dir(PACMAN_DIR)
        .map(|dir| dir.count() as i16)
        .ok()
}