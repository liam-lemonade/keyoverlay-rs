extern crate msgbox;
extern crate serde;

use std::fmt::Debug;

pub fn shutdown(code: i32) -> ! {
    std::process::exit(code);
}

pub fn messagebox(text: &str) {
    msgbox::create("KeyOverlay Daemon", text, msgbox::IconType::Info)
        .expect("Failed to create messagebox!");
}

pub fn handle_error<T: Debug>(text: &str, error: T) {
    let message = format!("{}\n\n{:?}", text, error);

    msgbox::create(
        "KeyOverlay Daemon",
        format!("An error occurred!\n{}", message).as_str(),
        msgbox::IconType::Error,
    )
    .expect("Failed to create messagebox!");
}
