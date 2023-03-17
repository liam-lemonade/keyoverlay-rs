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

    let title = format!("{} ({} v{})", crate::NAME, crate::BUILD, crate::VERSION);
    msgbox::create(title.as_str(), text, icon).unwrap();
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
