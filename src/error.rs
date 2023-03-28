extern crate anyhow;
extern crate native_dialog;

use native_dialog::{MessageDialog, MessageType};
use std::fmt::Debug;

pub enum ErrorStatus {
    FAILURE = 1,
    SUCCESS = 0,
}

pub fn display_message(text: &str, is_error: bool) {
    let message_type = if is_error {
        MessageType::Error
    } else {
        MessageType::Info
    };

    MessageDialog::new()
        .set_type(message_type)
        .set_title(crate::TITLE)
        .set_text(text)
        .show_alert()
        .expect("Failed to create message dialog");
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
