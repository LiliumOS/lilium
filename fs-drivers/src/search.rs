use core::num::NonZeroU32;

use crate::vobj::{ObjectId, StreamId};

pub trait Search {
    fn find_object(
        &mut self,
        dir: ObjectId,
        stream: StreamId,
        obj_name: &str,
    ) -> crate::io::Result<ObjectId>;
    fn find_stream(
        &mut self,
        obj: ObjectId,
        stream_name: &str,
        stream_idx: Option<NonZeroU32>,
    ) -> crate::io::Result<StreamId>;
}
