use std::{path::Path, fs::{create_dir_all, File}, io::Write};
use anyhow::{Result};

pub fn get_write(path: &Path) -> Result<impl Write> {
    create_dir_all(&path.parent().unwrap())?;
    Ok(File::create(&path)?)
}
