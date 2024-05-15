#![no_std]

mod dynloader;
mod elf;

use core::panic::PanicInfo;

use limine::{request::KernelAddressRequest, BaseRevision};

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static KERNEL_ADDRESS: KernelAddressRequest = KernelAddressRequest::new();

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    dynloader::init_dyn_loader(
        KERNEL_ADDRESS
            .get_response()
            .map(|x| x.virtual_base() as usize)
            .unwrap_or(0),
    );
    assert!(BASE_REVISION.is_supported());
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    // TODO: print info
    loop {}
}
