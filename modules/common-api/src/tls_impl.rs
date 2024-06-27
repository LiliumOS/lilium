use core::ffi::c_void;

#[cfg(target_arch = "x86_64")]
#[no_mangle]
#[naked]
extern "C" fn __tls_get_addr(_: *const c_void) -> *const c_void {
    unsafe { core::arch::asm!("lea rax, gs:[0]", "ret", options(noreturn)) }
}
