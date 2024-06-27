use core::ffi::c_void;

#[repr(usize)]
pub enum MemoryPool {
    /// Allocates memory that is always available to the kernel
    KernelImmediate = 0,
    KernelBuffer = 1,
    KernelUctx = 2,
    User = 0x800,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const PAGE_ATTR_PRESENT: usize = 0x00000001;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const PAGE_ATTR_READ: usize = 0x000000000;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const PAGE_ATTR_WRITE: usize = 0x00000002;

extern "system" {
    pub fn allocate_kernel_pages(base_hint: *mut c_void, page_count: usize) -> *mut c_void;
    pub fn deallocate_kernel_pages(page_addr: *mut c_void, page_count: usize);

    pub fn change_page_range_attrs(page_addr: *mut c_void, page_count: usize, new_attrs: usize);
}
