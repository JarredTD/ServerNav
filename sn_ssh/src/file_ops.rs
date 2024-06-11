use ssh2::Session;
use std::io::{Read, Write};
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

pub fn read_file(session: &Session, filepath: &PathBuf) -> Result<String, String> {
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

pub fn modify_file(session: &Session, filepath: &Path, content: &str) -> Result<(), String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {:?}", e))?;

    let mut remote_file = sftp
        .open_mode(
            filepath,
            ssh2::OpenFlags::WRITE,
            0o644,
            ssh2::OpenType::File,
        )
        .map_err(|e| format!("Failed to open file: {:?}", e))?;

    remote_file
        .write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write to file: {:?}", e))?;

    Ok(())
}

pub fn export_file(
    _session: &Session,
    _local_path: &Path,
    _remote_path: &Path,
) -> Result<(), String> {
    unimplemented!("Functionality to export files is not implemented yet.");
}

pub fn import_file(
    _session: &Session,
    _remote_path: &Path,
    _local_path: &Path,
) -> Result<(), String> {
    unimplemented!("Functionality to import files is not implemented yet.");
}
