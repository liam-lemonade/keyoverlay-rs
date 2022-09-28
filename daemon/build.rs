use windres::Build;

fn main() {
    Build::new().compile("tray-build.rc").unwrap();
}