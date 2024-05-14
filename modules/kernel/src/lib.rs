#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    // TODO: print info
    loop {}
}
