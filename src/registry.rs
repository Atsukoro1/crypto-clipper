extern crate winapi;

use std::io::Error;
use std::iter::once;
use std::{ffi::OsStr, os::windows::prelude::OsStrExt, ptr::null_mut};
use winapi::shared::minwindef::{HKEY, BYTE, DWORD};
use winapi::um::winnt::{REG_SZ, KEY_READ, KEY_WRITE};
use winapi::um::winreg::{RegSetValueExW, RegOpenKeyExW};
use winapi::{
    shared::{winerror::ERROR_SUCCESS},
    um::{
        winnt::KEY_ALL_ACCESS,
        winreg::{RegCreateKeyExW, HKEY_CURRENT_USER},
    },
};

#[allow(unused)]
pub fn create_registry_key(path: &str) -> Result<HKEY, std::io::Error> {
    unsafe {
        let key_path = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();

        let mut hkey: HKEY = null_mut();
        let mut disposition: u32 = 0;

        let result: i32 = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            key_path.as_ptr(),
            0,
            null_mut(),
            0,
            KEY_ALL_ACCESS,
            null_mut(),
            &mut hkey,
            &mut disposition,
        );

        if result == ERROR_SUCCESS.try_into().unwrap() {
            Ok(hkey)
        } else {
            Err(Error::from_raw_os_error(result))
        }
    }
}

pub fn set_registry_value(hkey: HKEY, value_name: &str, value_data: &str) -> Result<(), Error> {
    let value_name_wide = OsStr::new(value_name)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let value_data_wide = OsStr::new(value_data)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let result: i32 = unsafe {
        RegSetValueExW(
            hkey,
            value_name_wide.as_ptr(),
            0,
            REG_SZ,
            value_data_wide.as_ptr() as *const BYTE,
            (value_data_wide.len() * std::mem::size_of::<u16>()) as DWORD,
        )
    };

    if result == ERROR_SUCCESS.try_into().unwrap() {
        Ok(())
    } else {
        Err(Error::from_raw_os_error(result))
    }
}

pub fn open_registry_key(key_path: &str) -> Result<HKEY, Error> {
    let key_path_wide = OsStr::new(key_path)
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    let mut hkey: HKEY = null_mut();

    let result: i32 = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            key_path_wide.as_ptr(),
            0,
            KEY_READ | KEY_WRITE,
            &mut hkey,
        )
    };

    if result == ERROR_SUCCESS.try_into().unwrap() {
        Ok(hkey)
    } else {
        Err(Error::from_raw_os_error(result))
    }
}