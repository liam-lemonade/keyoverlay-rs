extern crate msgbox;

use std::fmt::Debug;

pub enum ExitStatus {
    Success = 0,
    Failure = 1,
}

pub fn shutdown(code: ExitStatus) -> ! {
    std::process::exit(code as i32);
}

pub fn messagebox(text: &str) {
    msgbox::create("keyoverlay-rs", text, msgbox::IconType::Info)
        .expect("Failed to create messagebox!");
}

pub fn handle_error<T: Debug>(text: &str, error: T) {
    let message = format!("{}\n\n{:?}", text, error);

    msgbox::create("keyoverlay-rs", message.as_str(), msgbox::IconType::Error)
        .expect("Failed to create messagebox!");
}
