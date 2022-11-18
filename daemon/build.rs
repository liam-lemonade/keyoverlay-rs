use windres::Build;

fn main() {
    if cfg!(windows) {
        Build::new().compile("tray-build.rc").unwrap();
    }
}
