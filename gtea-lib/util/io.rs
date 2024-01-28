use std::fs;

use crate::prelude::*;
use util::filepath;

pub fn mkdir(path: String) -> Result<bool> {
    if filepath::exists(&path) {
        return Ok(false);
    }

    fs::create_dir(path)?;

    Ok(true)
}

pub fn link(target: &str, path: &str) -> Result<()> {
    std::os::unix::fs::symlink(target, path)?;

    Ok(())
}
