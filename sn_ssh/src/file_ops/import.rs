use ssh2::Session;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn import_directory(
    session: &Session,
    remote_dir: &Path,
    local_dir: &Path,
) -> Result<(), String> {
    let local_dir_name = local_dir
        .file_name()
        .ok_or("Invalid local directory entry")?;
    let remote_subdir = remote_dir.join(local_dir_name);

    // Create the top-level remote directory
    let sftp = session
        .sftp()
        .map_err(|e| format!("Failed to create SFTP session: {}", e))?;

    sftp.mkdir(&remote_subdir, 0o755)
        .map_err(|e| format!("Failed to create remote directory: {}", e))?;

    import_directory_recursive(session, &sftp, &remote_subdir, local_dir)
}

fn import_directory_recursive(
    session: &Session,
    sftp: &ssh2::Sftp,
    remote_subdir: &Path,
    local_dir: &Path,
) -> Result<(), String> {
    for entry in
        fs::read_dir(local_dir).map_err(|e| format!("Failed to read local directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            let new_remote_subdir =
                remote_subdir.join(path.file_name().ok_or("Invalid local directory entry")?);
            sftp.mkdir(&new_remote_subdir, 0o755)
                .map_err(|e| format!("Failed to create remote directory: {}", e))?;
            import_directory_recursive(session, sftp, &new_remote_subdir, &path)?
        } else {
            let remote_file_path =
                remote_subdir.join(path.file_name().ok_or("Invalid local file entry")?);
            import_file(session, &remote_file_path, &path)?
        }
    }
    Ok(())
}

pub fn import_file(
    session: &Session,
    remote_directory: &Path,
    local_file_path: &Path,
) -> Result<(), String> {
    let local_file_name = local_file_path.file_name().ok_or("Invalid local path")?;
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
    let mut new_remote_path = remote_directory.join(local_file_name);

    // Check for the file's existence and increment the filename if necessary
    while sftp.stat(&new_remote_path).is_ok() {
        counter += 1;
        let new_file_name = format!("{}({}){}", file_base, counter, extension);
        new_remote_path = remote_directory.join(new_file_name);
    }

    let mut local_file =
        File::open(local_file_path).map_err(|e| format!("Failed to open local file: {}", e))?;
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
