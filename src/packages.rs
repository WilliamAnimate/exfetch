use std::path::Path;

const PACMAN_DIR: &str = "/var/lib/pacman/local";

fn folders_in_dir(dir: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.metadata()?.is_dir() {
                count += 1;
            }
        }
    }
    Ok(count)
}

pub fn get_num_packages() -> i16 {
    let directory = Path::new(PACMAN_DIR);
    match folders_in_dir(directory) {
        Ok(entries) => entries as i16,
        Err(_) => return 0
    }
}
