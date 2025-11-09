use leptos::prelude::WindowListenerHandle;

pub const SECONDS_UNTIL_INPUT_CLEARED: u8 = 30;

pub struct ScanInputManager {
    pub(crate) entries: Option<Vec<Entry>>,
}

impl Default for ScanInputManager {
    fn default() -> Self {
        Self {
            entries: Some(Vec::new()),
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub(crate) route: String,
    pub(crate) handle: WindowListenerHandle,
}
