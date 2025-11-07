use leptos::prelude::*;
use reactive_stores::Store;
use thaw::{
    Button, ButtonAppearance, ConfigProvider, Dialog, DialogActions, DialogBody, DialogContent,
    DialogSurface, DialogTitle,
};

use crate::frontend::model::throw_error::{ThrowError, THROW_ERROR_HARD};

#[component]
pub fn ErrorDisplay() -> impl IntoView {
    let store = expect_context::<Store<ThrowError<THROW_ERROR_HARD>>>();
    // needed because we need the effect to force execution on client
    let content = RwSignal::new(Vec::<String>::new());
    let open = RwSignal::new(false);
    Effect::new(move |_| {
        let error = store.get().0;
        open.set(!error.is_empty());
        content.set(error);
    });

    view! {
        <ConfigProvider>
            <Dialog open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>"An error occured"</DialogTitle>
                        <DialogContent>
                            {move || {
                                content
                                    .get()
                                    .into_iter()
                                    .map(|e| view! { <p>{e}</p> })
                                    .collect_view()
                            }}
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| {
                                    *store.write() = ThrowError(vec![]);
                                }
                            >
                                "Ok"
                            </Button>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </ConfigProvider>
    }
    .into_any()
}
