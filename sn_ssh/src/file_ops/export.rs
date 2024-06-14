use ssh2::Session;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn export_directory(session: &Session, remote_dir: &Path) -> Result<(), String> {
    let downloads_folder = match dirs::download_dir() {
        Some(path) => path,
        None => return Err("Could not determine downloads folder".to_string()),
    };

    let remote_dir_name = remote_dir
        .file_name()
        .ok_or("Invalid remote directory entry")?;
    let local_dir = downloads_folder.join(remote_dir_name);

    // Create the top-level directory
    if let Err(e) = fs::create_dir_all(&local_dir) {
        return Err(format!("Failed to create local directory: {}", e));
    }

    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    export_directory_recursive(session, &sftp, remote_dir, &local_dir)
}

fn export_directory_recursive(
    session: &Session,
    sftp: &ssh2::Sftp,
    remote_dir: &Path,
    local_dir: &Path,
) -> Result<(), String> {
    for entry in sftp
        .readdir(remote_dir)
        .map_err(|e| format!("Failed to read remote directory: {}", e))?
    {
        let (path, _) = entry;
        let remote_path = remote_dir.join(&path);

        let stat = sftp.stat(&remote_path);
        let stat = match stat {
            Ok(stat) => stat,
            Err(e) => return Err(format!("Failed to stat remote path: {}", e)),
        };

        if stat.is_dir() {
            let local_subdir =
                local_dir.join(path.file_name().ok_or("Invalid remote directory entry")?);
            if let Err(e) = fs::create_dir_all(&local_subdir) {
                return Err(format!("Failed to create local directory: {}", e));
            }
            export_directory_recursive(session, sftp, &remote_path, &local_subdir)?
        } else {
            let local_file_path =
                local_dir.join(path.file_name().ok_or("Invalid remote file entry")?);
            export_file(session, &remote_path, &local_file_path)?
        }
    }
    Ok(())
}

pub fn export_file(
    session: &Session,
    remote_file_path: &Path,
    local_file_path: &Path,
) -> Result<(), String> {
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
        File::create(local_file_path).map_err(|e| format!("Failed to create local file: {}", e))?;

    local_file
        .write_all(&buffer)
        .map_err(|e| format!("Failed to write to local file: {}", e))?;

    Ok(())
}
