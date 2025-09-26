use leptos::prelude::*;
use reactive_stores::Store;

use crate::frontend::model::frontend_store::{FrontendStore, FrontendStoreStoreFields};

pub fn throw_error(str: impl ToString) {
    let store = expect_context::<Store<FrontendStore>>();
    store.error().write().update(|old| {
        old.push(str.to_string());
    });
}
