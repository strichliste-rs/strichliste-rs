use leptos::prelude::*;
use reactive_stores::Store;
use thaw::{Toast, ToastBody, ToastIntent, ToastOptions, ToastTitle, ToasterInjection};

use crate::frontend::model::throw_error::{ThrowError, THROW_ERROR_SOFT};

#[component]
pub fn ErrorSoftDisplay() -> impl IntoView {
    let store = expect_context::<Store<ThrowError<THROW_ERROR_SOFT>>>();
    let toaster = ToasterInjection::expect_context();
    Effect::new(move |_| {
        let error = store.get().0;
        let error_empty = error.is_empty();
        for e in error {
            toaster.dispatch_toast(
                move || {
                    view! {
                        <Toast>
                            <ToastTitle>"An Error occured"</ToastTitle>
                            <ToastBody>{e.to_string()}</ToastBody>
                        </Toast>
                    }
                },
                ToastOptions::default().with_intent(ToastIntent::Warning),
            );
        }
        if !error_empty {
            store.set(ThrowError(vec![]));
        }
    });
    ().into_any();
}
