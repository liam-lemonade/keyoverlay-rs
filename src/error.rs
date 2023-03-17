extern crate anyhow;
extern crate msgbox;

use msgbox::IconType;
use std::fmt::Debug;

pub enum ErrorStatus {
    FAILURE = 1,
    SUCCESS = 0,
}

pub fn display_message(text: &str, is_error: bool) {
    let icon = if is_error {
        IconType::Error
    } else {
        IconType::Info
    };

    msgbox::create(crate::TITLE, text, icon).unwrap();
}

pub fn display_error<T: Debug>(thread_name: &str, error_data: T) {
    let message = format!(
        "An error occured while running the {} thread\n\n{:?}",
        thread_name, error_data
    );

    self::display_message(message.as_str(), true);
}

pub fn shutdown(status: ErrorStatus) -> ! {
    std::process::exit(status as i32);
}
