// SPDX-License-Identifier: MIT

use libc::{sysconf, _SC_PAGESIZE};
use std::os::raw::c_long;

#[cfg(feature = "gui")]
use rfd::{MessageButtons, MessageDialog, MessageLevel};
#[cfg(feature = "gui")]
use std::io::{stdin, IsTerminal};

pub fn get_page_size() -> Option<usize> {
    unsafe {
        let page_size: c_long = sysconf(_SC_PAGESIZE);
        if page_size == -1 {
            None // Error retrieving page size
        } else {
            Some(page_size as usize)
        }
    }
}

#[cfg(feature = "gui")]
fn dialog(msg: &str, level: MessageLevel) {
    if !stdin().is_terminal() {
        MessageDialog::new()
            .set_title("binfmt-dispatcher")
            .set_description(msg)
            .set_level(level)
            .set_buttons(MessageButtons::Ok)
            .show();
    }
}

#[cfg(feature = "gui")]
pub fn error_dialog(msg: &str) {
    dialog(msg, MessageLevel::Error);
}

#[cfg(feature = "gui")]
pub fn warn_dialog(msg: &str) {
    dialog(msg, MessageLevel::Warning);
}
