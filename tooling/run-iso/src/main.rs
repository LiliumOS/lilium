#![feature(exit_status_error)]

use std::process::Command;

fn main() {
    Command::new("cargo")
        .arg("iso")
        .status()
        .expect("iso build failed")
        .exit_ok()
        .expect("iso build failed");
    
    Command::new("qemu-system-x86_64")
        .args([
            "-M",
            "q35",
            "-m",
            "2G",
            "-bios",
            "ovmf/OVMF.fd",
            "-cdrom",
            "lilium.iso",
            "-boot",
            "d",
        ])
        .status()
        .expect("error running iso")
        .exit_ok()
        .expect("error running iso");
}
