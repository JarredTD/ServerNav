use ssh2::Session;
use std::io::Read;
use std::path::Path;

pub fn list_dir(session: &Session, directory: &str) -> Result<Vec<String>, String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;
    let mut dir = sftp
        .opendir(Path::new(directory))
        .map_err(|e| format!("Failed to open directory: {}", e))?;

    let mut filenames = Vec::new();
    while let Ok((path, _)) = dir.readdir() {
        if let Some(name) = path.file_name() {
            filenames.push(name.to_string_lossy().into_owned());
        }
    }
    Ok(filenames)
}

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

pub fn get_working_dir(session: &Session) -> Result<String, String> {
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    let pwd = sftp
        .realpath(Path::new("."))
        .map_err(|e| format!("Failed to get PWD: {}", e))?;

    Ok(pwd.to_string_lossy().into_owned())
}
