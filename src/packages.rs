use std::path::Path;

const PACMAN_DIR: &str = "/var/lib/pacman/local";

#[inline(always)]
fn folders_in_dir(dir: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    std::fs::read_dir(dir)?.try_for_each(|entry| {
        if entry.unwrap().metadata()?.is_dir() {
            count += 1;
        }
        Ok::<(), std::io::Error>(())
    })?;
    Ok(count)
}

pub fn get_num_packages() -> usize {
    let directory = Path::new(PACMAN_DIR);
    folders_in_dir(directory).unwrap_or(0)
}

