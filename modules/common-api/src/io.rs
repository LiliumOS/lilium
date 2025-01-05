use crate::handle::HandleData;

#[repr(C, align(16))]
pub struct IoData {
    objdata: *mut (),
    optimistic_io_size: usize,
}

unsafe impl HandleData for IoData {
    const HANDLE_TYPE: usize = 3;
    const USE_SUBTYPE_MASK: usize = 0xF0000000;
}
