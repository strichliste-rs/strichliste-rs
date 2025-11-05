use leptos::prelude::RwSignal;

pub struct CachingEntry<T> {
    pub value: RwSignal<T>,
    pub is_fetching: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

impl<T> Default for CachingEntry<T>
where
    T: Sync + Default + Send + 'static,
{
    fn default() -> Self {
        Self {
            value: RwSignal::new(Default::default()),
            is_fetching: RwSignal::new(false),
            error: RwSignal::new(None),
        }
    }
}
