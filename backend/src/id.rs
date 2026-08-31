use crate::PluxerApi;

pub trait BackendId {
    type Api: PluxerApi;

    fn as_snowflake(&self) -> Option<u64>;
}
