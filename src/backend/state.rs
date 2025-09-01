#[cfg(feature = "ssr")]
use {
    crate::backend::{db::DB, Settings},
    std::sync::Arc,
    tokio::sync::Mutex,
};

pub type ServerState = Arc<State>;

#[cfg(feature = "ssr")]
pub struct State {
    pub db: Mutex<DB>,
    pub settings: Settings,
}
