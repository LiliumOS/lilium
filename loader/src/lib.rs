#![no_std]

mod dynloader;
mod elf;

use core::panic::PanicInfo;

use limine::BaseRevision;

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    dynloader::init_dyn_loader(0);
    assert!(BASE_REVISION.is_supported());
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    // TODO: print info
    loop {}
}
