extern crate winapi;

use core::slice;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::TRUE;
use winapi::um::winuser::{
    CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
    CF_UNICODETEXT,
};

pub fn set_clipboard_text(text: &str) -> bool {
    let mut result = false;

    let text_wide: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    let text_len_bytes = text_wide.len() * std::mem::size_of::<u16>();

    unsafe {
        if OpenClipboard(null_mut()) == TRUE {
            if EmptyClipboard() == TRUE {
                let h_mem = winapi::um::winbase::GlobalAlloc(
                    winapi::um::winbase::GMEM_MOVEABLE | winapi::um::winbase::GMEM_DDESHARE,
                    text_len_bytes,
                );

                if !h_mem.is_null() {
                    let mem_ptr = winapi::um::winbase::GlobalLock(h_mem) as *mut u16;
                    std::ptr::copy_nonoverlapping(text_wide.as_ptr(), mem_ptr, text_wide.len());
                    winapi::um::winbase::GlobalUnlock(h_mem);

                    if SetClipboardData(CF_UNICODETEXT, h_mem).is_null() {
                        winapi::um::winbase::GlobalFree(h_mem);
                    } else {
                        result = true;
                    }
                }
            }
            CloseClipboard();
        }
    }

    result
}

pub fn get_clipboard_text() -> Option<String> {
    let mut result: Option<String> = None;

    unsafe {
        if OpenClipboard(null_mut()) == TRUE {
            let h_mem = GetClipboardData(CF_UNICODETEXT);
            if !h_mem.is_null() {
                let mem_ptr = winapi::um::winbase::GlobalLock(h_mem) as *const u16;
                let text_wide = wide_ptr_to_string(mem_ptr);
                result = Some(text_wide);

                winapi::um::winbase::GlobalUnlock(h_mem);
            }
            CloseClipboard();
        }
    }

    result
}

fn wide_ptr_to_string(ptr: *const u16) -> String {
    let len = (0..)
        .take_while(|&i| unsafe { *ptr.offset(i) } != 0)
        .count();
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    String::from_utf16_lossy(slice)
}
