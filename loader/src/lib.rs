#![no_std]

use core::panic::PanicInfo;

use limine::BaseRevision;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    // TODO: print info
    loop {}
}
