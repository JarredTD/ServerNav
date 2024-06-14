use ssh2::Session;
use std::path::{Path, PathBuf};

pub fn list_dir(
    session: &Session,
    directory: &PathBuf,
) -> Result<Vec<(PathBuf, ssh2::FileStat)>, String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    let mut dir = sftp
        .opendir(directory)
        .map_err(|e| format!("Failed to open directory: {}", e))?;

    let mut paths = Vec::new();
    while let Ok((path, stat)) = dir.readdir() {
        let full_path = Path::new(directory).join(path);
        paths.push((full_path, stat));
    }
    Ok(paths)
}

pub fn get_working_dir(session: &Session) -> Result<PathBuf, String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    let pwd = sftp
        .realpath(Path::new("."))
        .map_err(|e| format!("Failed to get PWD: {}", e))?;

    Ok(pwd)
}
