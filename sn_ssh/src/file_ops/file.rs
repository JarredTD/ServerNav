use ssh2::Session;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

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

pub fn modify_file(session: &Session, filepath: &Path, content: &str) -> Result<(), String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {:?}", e))?;

    let mut remote_file = sftp
        .open_mode(
            filepath,
            ssh2::OpenFlags::WRITE | ssh2::OpenFlags::TRUNCATE,
            0o644,
            ssh2::OpenType::File,
        )
        .map_err(|e| format!("Failed to open file: {:?}", e))?;

    remote_file
        .write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write to file: {:?}", e))?;

    remote_file
        .flush()
        .map_err(|e| format!("Failed to flush file buffer: {:?}", e))?;

    remote_file
        .close()
        .map_err(|e| format!("Failed to close file: {:?}", e))?;

    Ok(())
}
