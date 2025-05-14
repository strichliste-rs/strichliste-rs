#[cfg(feature = "ssr")]
use {crate::backend::db::DB, std::sync::Arc, tokio::sync::Mutex};

pub type ServerState = Arc<State>;

#[cfg(feature = "ssr")]
pub struct State {
    pub db: Mutex<DB>,
}
