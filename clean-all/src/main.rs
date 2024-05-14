use std::fs;

fn main() {
    fs::remove_dir_all("target").ok();
    fs::remove_dir_all("loader/target").ok();
    fs::remove_dir_all("modules/target").ok();
}
