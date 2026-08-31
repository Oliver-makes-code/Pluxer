use crate::PluxerApi;

pub trait BackendId: Send + Sync {
    type Api: PluxerApi;

    fn as_snowflake(&self) -> Option<u64>;
}
