use std::sync::Once;

mod model;
mod manifest;
mod membership;
mod api;
mod hash;

static INIT_ENV_LOG: Once = Once::new();

pub(crate) fn init_log() {
    INIT_ENV_LOG.call_once(|| {
        env_logger::init();
    });
}