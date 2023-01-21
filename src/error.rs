use std::fmt::Debug;

pub enum ErrorCode {
    Success = 0,
    Failure = 1,
}

pub fn msgbox<T: Debug>(text: &str, error: T) {
    msgbox::create(
        "keyoverlay-rs",
        format!("{}\n\n{:?}", text, error).as_str(),
        msgbox::IconType::Error,
    )
    .expect("Failed to create messagebox");
}

pub fn shutdown(code: ErrorCode) -> ! {
    std::process::exit(code as i32);
}
