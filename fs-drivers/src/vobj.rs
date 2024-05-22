pub use libliliumfs::object::{ObjectId, StreamId};
use libliliumfs::uuid::Uuid;

#[repr(transparent)]
pub struct VolumeId(pub Uuid);

pub struct VrefId {
    pub volume: VolumeId,
    pub object: ObjectId,
    pub stream: StreamId,
}

pub mod acl;
