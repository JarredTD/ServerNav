use ssh2::Session;
use std::io::Read;
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

//TODO Untested, modifying files over session to be implemented later
pub fn read_file(session: &Session, filepath: &str) -> Result<String, String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;
    let mut file = sftp
        .open(Path::new(filepath))
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    Ok(contents)
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
