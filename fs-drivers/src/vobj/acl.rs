pub use libliliumfs::object::LegacySecurityDescriptor;

pub struct Acl {
    legacy: Option<LegacySecurityDescriptor>,
}
