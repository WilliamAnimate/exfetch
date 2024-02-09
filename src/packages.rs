// Made by William Animate [https://github.com/WilliamAnimate]
use std::{fs, path::Path};
use std::fs::read_dir;

const PACMAN_DIR: &str = "/var/lib/pacman/local";

pub fn get_num_packages() -> i16 {
    read_dir(Path::new(PACMAN_DIR)).unwrap().count() as i16
}
