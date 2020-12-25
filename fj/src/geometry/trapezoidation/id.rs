pub trait NodeId {
    fn raw_id(&self) -> RawId;
}

pub type RawId = u32;

impl NodeId for RawId {
    fn raw_id(&self) -> RawId {
        *self
    }
}
