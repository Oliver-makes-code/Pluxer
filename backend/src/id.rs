use std::hash::Hash;

use crate::PluxerApi;

pub trait BackendId: Send + Sync + Eq + Hash {
    type Api: PluxerApi;

    fn as_snowflake(&self) -> Option<u64>;
}
