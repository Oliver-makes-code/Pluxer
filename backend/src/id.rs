use std::hash::Hash;

use pluxer_database::platform_id::PlatformId;

use crate::PluxerApi;

pub trait BackendId: Send + Sync + Eq + Hash + Clone + 'static {
    type Api: PluxerApi;

    fn to_platform_id(&self, instance_name: Option<&str>) -> PlatformId;
    fn from_platform_id(value: &PlatformId) -> Option<Self>;
}
