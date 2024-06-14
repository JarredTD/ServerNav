use ssh2::Session;
use std::fs::{self, File};
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
    remote_file_path: &Path,
    local_path: &Path,
) -> Result<(), String> {
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
    let mut new_remote_path = remote_file_path.to_path_buf();

    // Check for the file's existence and increment the filename if necessary
    while sftp.stat(&new_remote_path).is_ok() {
        counter += 1;
        let new_file_name = format!("{}({}){}", file_base, counter, extension);
        new_remote_path = remote_file_path.with_file_name(new_file_name);
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
