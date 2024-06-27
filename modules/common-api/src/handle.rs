#[repr(C)]
#[cfg_attr(target_pointer_width = "64", repr(align(32)))]
#[cfg_attr(target_pointer_width = "32", repr(align(16)))]
pub struct Handle<D> {
    pub handle_type: usize,
    pub handle_metadata: *mut ObjectMetadata,
    pub handle_rights: *mut HandleRights,
    pub data: *mut D,
}

pub unsafe trait HandleData {
    const HANDLE_TYPE: usize;
    const USE_SUBTYPE_MASK: usize = 0;
}

pub struct ObjectMetadata {}

pub struct HandleRights {}
