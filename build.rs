use cc;

fn main() {
    if cfg!(target_os = "macos") {
        cc::Build::new().file("src/c/utun.c").compile("utun");
    }
}
