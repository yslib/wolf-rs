use std::{
    fs::File,
    path::{Path, PathBuf},
};
pub fn app_root_dir() -> Result<PathBuf, std::io::Error> {
    // if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
    //     return Ok(PathBuf::from(dir));
    // }
	return std::env::current_dir();
    let mut exe_dir = std::env::current_exe()?;
    if exe_dir.pop() {
        return Ok(exe_dir);
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to find app root dir",
    ))
}

pub fn asset_file<T: AsRef<Path>>(path: T) -> std::io::Result<File> {
    let mut root_dir = app_root_dir()?;
    root_dir.push(path);
    std::fs::File::open(root_dir)
}

pub fn read_asset_as_binary<T: AsRef<Path>>(path: T) -> std::io::Result<Vec<u8>> {
    let mut root_dir = app_root_dir()?;
    root_dir.push(path);
    std::fs::read(root_dir)
}