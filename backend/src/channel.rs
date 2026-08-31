use crate::PluxerApi;

pub trait BackendChannel {
    type Api: PluxerApi;
}
