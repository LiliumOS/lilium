#[cfg(target_arch = "x86_64")]
pub fn halt() -> ! {
    unsafe { core::arch::asm!("2: hlt", "jmp 2b", options(noreturn)) }
}
