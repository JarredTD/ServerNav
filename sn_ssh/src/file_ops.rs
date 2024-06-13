use ssh2::Session;
use std::fs::File;
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

pub fn import_file(session: &Session, remote_dir: &Path, local_path: &Path) -> Result<(), String> {
    let local_file_name = local_path.file_name().ok_or("Invalid local path")?;
    let local_file_stem = local_file_name
        .to_str()
        .ok_or("Failed to convert file name to string")?;
    let mut file_base = local_file_stem.to_string();
    let mut extension = String::new();

    if let Some(pos) = local_file_stem.rfind('.') {
        file_base = local_file_stem[..pos].to_string();
        extension = local_file_stem[pos..].to_string();
    }

    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;
    let mut counter = 0;
    let mut new_remote_path = remote_dir.join(local_file_name);

    // Check for the file's existence and increment the filename if necessary
    while sftp.stat(&new_remote_path).is_ok() {
        counter += 1;
        let new_file_name = format!("{}({}){}", file_base, counter, extension);
        new_remote_path = remote_dir.join(new_file_name);
    }

    let mut local_file =
        File::open(local_path).map_err(|e| format!("Failed to open local file: {}", e))?;
    let mut buffer = Vec::new();

    local_file
        .read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read local file: {}", e))?;

    let mut remote_file = session
        .scp_send(&new_remote_path, 0o644, buffer.len() as u64, None)
        .map_err(|e| format!("Failed to start SCP send: {}", e))?;

    remote_file
        .write_all(&buffer)
        .map_err(|e| format!("Failed to write to remote file: {}", e))?;

    Ok(())
}

pub fn export_file(session: &Session, remote_file_path: &Path) -> Result<(), String> {
    let downloads_folder = match dirs::download_dir() {
        Some(path) => path,
        None => return Err("Could not determine downloads folder".to_string()),
    };

    let remote_file_name = remote_file_path
        .file_name()
        .ok_or("Invalid remote file path")?;
    let local_file_path = downloads_folder.join(remote_file_name);

    let local_file_stem = remote_file_name
        .to_str()
        .ok_or("Failed to convert file name to string")?;
    let mut file_base = local_file_stem.to_string();
    let mut extension = String::new();

    if let Some(pos) = local_file_stem.rfind('.') {
        file_base = local_file_stem[..pos].to_string();
        extension = local_file_stem[pos..].to_string();
    }

    // Check if the local file exists and increment the filename if necessary
    let mut counter = 0;
    let mut new_local_path = local_file_path.clone();

    while new_local_path.exists() {
        counter += 1;
        let new_file_name = format!("{}({}){}", file_base, counter, extension);
        new_local_path = downloads_folder.join(new_file_name);
    }

    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    let mut remote_file = sftp
        .open(remote_file_path)
        .map_err(|e| format!("Failed to open remote file: {}", e))?;

    let mut buffer = Vec::new();
    remote_file
        .read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read remote file: {}", e))?;

    let mut local_file =
        File::create(new_local_path).map_err(|e| format!("Failed to create local file: {}", e))?;

    local_file
        .write_all(&buffer)
        .map_err(|e| format!("Failed to write to local file: {}", e))?;

    Ok(())
}
