use leptos::{leptos_dom::logging::console_log, prelude::*};
use reactive_stores::Store;

use crate::frontend::model::frontend_store::{FrontendStore, FrontendStoreStoreFields};

pub fn throw_error(str: impl ToString) {
    let store = expect_context::<Store<FrontendStore>>();
    let msg = str.to_string();
    console_log(&msg);
    store.error().write().update(|old| {
        old.push(msg);
    });
}

pub fn throw_error_none_view(str: impl ToString) -> AnyView {
    throw_error(str);
    ().into_any()
}
