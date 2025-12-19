use leptos::{
    component,
    prelude::{Effect, Get, GetUntracked, RwSignal, Set, Signal, IntoAnyAttribute},
    view, IntoView,
};
use leptos_fluent::move_tr;
use leptos_meta::Style;
use thaw::{
    Button, ComponentRef, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface,
    DialogTitle, Field, Icon, Input, InputPrefix, InputRef, InputType,
};

use crate::frontend::component::figma::{colors::Color, spacings::Spacing};
use leptos::prelude::AddAnyAttr;

pub enum InputDialogType {
    Text,
    Password,
}

#[component]
pub fn InputDialog(
    title: Signal<String>,
    description: Signal<String>,
    ok_button_text: Signal<String>,
    open_signal: RwSignal<bool>,
    input_dialog_type: InputDialogType,
    on_ok: impl Fn(String) + Send + Sync + 'static,
) -> impl IntoView {
    let comp_ref = ComponentRef::<InputRef>::new();
    let input_value = RwSignal::new(String::new());
    Effect::new(move || {
        if let Some(comp_ref) = comp_ref.get() {
            if open_signal.get() {
                input_value.set(String::new());
                comp_ref.focus();
            }
        }
    });

    view! {
        <Dialog open=open_signal>
            <DialogSurface style:background="rgba(0, 0, 0, 0)">
                <DialogBody
                    style:background-color=Color::BACKGROUND_DARK
                    style:border-radius=Spacing::XS
                >
                    <DialogTitle
                        style:font-size=Spacing::M
                        style:font-weight="400"
                        style:padding=Spacing::px_to_rem(30)
                        style:padding-left=Spacing::S
                        style:background-color=Color::BACKGROUND_LIGHT
                        style:border-radius=format!("{0} {0} 0 0", Spacing::XS)
                        style:box-shadow="0 4px 4px #00000040"
                    >
                        {title}
                    </DialogTitle>
                    <DialogContent
                        style:padding-left=Spacing::S
                        style:padding-right=Spacing::S
                        style:padding-top=Spacing::S
                    >
                        <Style>
                            {format!(
                                r#"
                                    .thaw-label {{
                                        font-size: {0} !important;
                                        padding-top: {1} !important;
                                        padding-bottom: {1} !important;
                                    }}
                                "#,
                                Spacing::M,
                                Spacing::S,
                            )}
                        </Style>
                        <Field label=description>
                            <Input
                                style:font-size=Spacing::M
                                style:border-radius=Spacing::XS
                                style:border-width=Spacing::px_to_rem_f(2.5)
                                style:height=Spacing::px_to_rem(81)
                                style:max-width=Spacing::px_to_rem(540)
                                comp_ref
                                value=input_value
                                input_type=match input_dialog_type {
                                    InputDialogType::Text => InputType::Text,
                                    InputDialogType::Password => InputType::Password,
                                }
                            >
                                <InputPrefix slot>
                                    <Icon style:width=Spacing::L icon=icondata::LuUser />
                                </InputPrefix>
                            </Input>
                        </Field>
                    </DialogContent>
                    <DialogActions
                        style:padding-bottom=Spacing::S
                        style:padding-right=Spacing::S
                        style:padding-top=Spacing::L
                    >
                        <Button
                            style:height=Spacing::px_to_rem(81)
                            style:width=Spacing::px_to_rem(183)
                            style:font-size=Spacing::M
                            style:font-weight="400"
                            style:border="none"
                            style:padding=format!("{} {}", Spacing::S, Spacing::M)
                            on_click=move |_| { open_signal.set(false) }
                        >
                            {move_tr!("component-InputDialog-button-cancel")}
                        </Button>
                        <Button
                            style:height=Spacing::px_to_rem(81)
                            style:width=Spacing::px_to_rem(183)
                            style:border="none"
                            style:font-size=Spacing::M
                            style:border-radius=Spacing::XS
                            style:font-weight="400"
                            style:background=Color::PRIMARY
                            style:padding=format!("{} {}", Spacing::S, Spacing::M)
                            style:margin-left=Spacing::px_to_rem(10)
                            on_click=move |_| {
                                let value = input_value.get_untracked();
                                if value.is_empty() {
                                    return;
                                }
                                on_ok(value)
                            }
                        >
                            {ok_button_text}
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
