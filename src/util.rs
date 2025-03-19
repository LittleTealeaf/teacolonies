use std::io;
use std::path::Path;
use std::{
    fs::{self, File},
    path::PathBuf,
};

use zip::ZipArchive;

pub fn unzip(zip_path: &str, dest_dir: &str) -> io::Result<()> {
    let dest_path = Path::new(dest_dir);
    if dest_path.exists() {
        fs::remove_dir_all(dest_path)?;
    }

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = PathBuf::from(dest_dir).join(file.name());

        if file.unix_mode().map(|m| m & 0o120000 == 0o120000).unwrap_or(false) {  // 0o120000 is the symlink mode
            println!("Skipping symbolic link: {}", file.name());
            continue;
        }

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}
