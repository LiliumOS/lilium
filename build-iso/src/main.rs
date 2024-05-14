#![feature(exit_status_error)]

use std::{fs, path::PathBuf, process::Command};

fn make_limine() {
    Command::new("make")
        .args(["-C", "limine"])
        .status()
        .expect("limine make failed")
        .exit_ok()
        .expect("limine make failed");
}

fn build_lilium_crates() {
    Command::new("cargo")
        .arg("build")
        .current_dir(PathBuf::from("loader").canonicalize().unwrap())
        .status()
        .expect("loader build failed")
        .exit_ok()
        .expect("loader build failed");

    Command::new("cargo")
        .arg("build")
        .current_dir(PathBuf::from("modules").canonicalize().unwrap())
        .status()
        .expect("modules build failed")
        .exit_ok()
        .expect("modules build failed");
}

fn build_iso_dir() {
    fs::remove_dir_all("iso-root").ok();

    fs::create_dir_all("iso-root/boot/sys").unwrap();
    fs::copy(
        "loader/target/x86_64-pc-lilium-loader/debug/liblilium_loader.so",
        "iso-root/boot/lilium-loader.so",
    )
    .unwrap();
    fs::copy(
        "modules/target/x86_64-pc-lilium-kernel/debug/liblilium_kernel.so",
        "iso-root/boot/sys/lilium-kernel.so",
    )
    .unwrap();

    fs::create_dir("iso-root/boot/limine").unwrap();
    fs::copy("limine.cfg", "iso-root/boot/limine/limine.cfg").unwrap();
    fs::copy(
        "limine/limine-bios.sys",
        "iso-root/boot/limine/limine-bios.sys",
    )
    .unwrap();
    fs::copy(
        "limine/limine-bios-cd.bin",
        "iso-root/boot/limine/limine-bios-cd.bin",
    )
    .unwrap();
    fs::copy(
        "limine/limine-uefi-cd.bin",
        "iso-root/boot/limine/limine-uefi-cd.bin",
    )
    .unwrap();

    fs::create_dir_all("iso-root/EFI/BOOT").unwrap();
    fs::copy("limine/BOOTX64.EFI", "iso-root/EFI/BOOT/BOOTX64.EFI").unwrap();
    fs::copy("limine/BOOTIA32.EFI", "iso-root/EFI/BOOT/BOOTIA32.EFI").unwrap();
}

fn build_iso() {
    Command::new("xorriso")
        .args([
            "-as",
            "mkisofs",
            "-b",
            "boot/limine/limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "boot/limine/limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            "iso-root",
            "-o",
            "lilium.iso",
        ])
        .status()
        .expect("iso build failed")
        .exit_ok()
        .expect("iso build failed");

    Command::new("limine/limine")
        .args(["bios-install", "lilium.iso"])
        .status()
        .expect("lilium bios install failed")
        .exit_ok()
        .expect("lilium bios install failed");

    fs::remove_dir_all("iso-root").ok();
}

fn main() {
    fs::remove_file("lilium.iso").ok(); // Just in case

    make_limine();
    build_lilium_crates();
    build_iso_dir();
    build_iso();
}
