use leptos::{leptos_dom::logging::console_log, prelude::*};
use reactive_stores::Store;

use crate::frontend::model::throw_error::{ThrowError, THROW_ERROR_HARD, THROW_ERROR_SOFT};

pub fn _throw_error<const L: u8>(str: impl ToString) {
    let store = expect_context::<Store<ThrowError<L>>>();
    let msg = str.to_string();
    console_log(&msg);
    store.update(|old| {
        old.0.push(msg);
    });
}
pub fn throw_error(str: impl ToString) {
    _throw_error::<THROW_ERROR_HARD>(str);
}

pub fn throw_error_none_view(str: impl ToString) -> AnyView {
    throw_error(str);
    ().into_any()
}

pub fn throw_error_soft(str: impl ToString) {
    _throw_error::<THROW_ERROR_SOFT>(str);
}

pub fn throw_error_soft_none_view(str: impl ToString) -> AnyView {
    throw_error_soft(str);
    ().into_any()
}
