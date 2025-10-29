use crate::{
    backend::core::behaviour::{
        update_user::UpdateUser, user_get::get_user, user_preferences_get::get_user_preferences,
        user_preferences_set::set_user_preferences,
    },
    frontend::{
        component::return_to::ReturnTo,
        route::user::RETURN_TO_MAIN_VIEW_TIMEOUT_SEC,
        shared::{throw_error, throw_error_none_view},
    },
    model::UserId,
};
use leptos::{
    ev::{self},
    prelude::*,
    reactive::spawn_local,
};
use leptos_router::hooks::use_params_map;
use thaw::{
    Button, ButtonSize, ButtonType, Field, FieldContextInjection, FieldContextProvider, Flex,
    FlexAlign, FlexGap, FlexJustify, Input, InputRule, Spinner, Switch, Toast, ToastBody,
    ToastTitle, ToasterInjection,
};

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(user_id) => UserId(user_id),
        Err(_) => {
            return throw_error_none_view(format!(
                "Failed to convert id: {user_id_string} to a number!"
            ));
        }
    };

    let user_resource = OnceResource::new(get_user(user_id));
    let preferences_resource = OnceResource::new(get_user_preferences(user_id));

    let update_action = ServerAction::<UpdateUser>::new();

    // prevents the form from submitting if a user uses a HID to input a barcode
    let enter_handler = window_event_listener(ev::keypress, move |ev| {
        if ev.key().as_str() == "Enter" {
            ev.prevent_default();
        }
    });

    on_cleanup(move || {
        enter_handler.remove();
    });
    view! {
        <ReturnTo after=RETURN_TO_MAIN_VIEW_TIMEOUT_SEC route="/" />
        <Suspense fallback=move || {
            view! {
                <div class="pt-5">
                    <Spinner label="Loading user" />
                </div>
            }
        }>
            {move || {
                let user = match user_resource.get() {
                    None => return ().into_any(),
                    Some(user) => {
                        match user {
                            Ok(user) => {
                                match user {
                                    Some(user) => user,
                                    None => {
                                        return throw_error_none_view(
                                            format!("No such user with id '{}' found!", user_id),
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                return throw_error_none_view(format!("Failed to fetch user: {e}"));
                            }
                        }
                    }
                };
                let preferences = match preferences_resource.get() {
                    None => return ().into_any(),
                    Some(value) => {
                        match value {
                            Ok(value) => RwSignal::new(value),
                            Err(e) => {
                                return throw_error_none_view(
                                    format!("Failed to fetch preferences: {e}"),
                                );
                            }
                        }
                    }
                };
                let alternative_color_signal = RwSignal::new(
                    preferences.get_untracked().alternative_coloring,
                );
                Effect::new(move || {
                    if let Some(Err(e)) = update_action.value().get() {
                        throw_error(format!("Failed to update user: {e}"));
                    }
                });
                let toaster = ToasterInjection::expect_context();
                Effect::new(move || {
                    let alternative_color = alternative_color_signal.get();
                    if alternative_color == preferences.get_untracked().alternative_coloring {
                        return;
                    }
                    let old_preferences = preferences.get_untracked();
                    preferences.update(|value| value.alternative_coloring = alternative_color);
                    spawn_local(async move {
                        match set_user_preferences(user_id, preferences.get_untracked()).await {
                            Ok(_) => {
                                toaster
                                    .dispatch_toast(
                                        move || {

                                            // nothing has changed

                                            view! {
                                                <Toast>
                                                    <ToastTitle>"Updated preferences!"</ToastTitle>
                                                    <ToastBody>
                                                        "Successfully updated your preferences"
                                                    </ToastBody>
                                                </Toast>
                                            }
                                        },
                                        Default::default(),
                                    )
                            }
                            Err(e) => {
                                alternative_color_signal.set(!alternative_color);
                                preferences.update(|value| *value = old_preferences);
                                throw_error(format!("Failed to update preferences: {e}"))
                            }
                        }
                    })
                });

                view! {
                    <div class="pt-5">
                        <Flex
                            justify=FlexJustify::Center
                            align=FlexAlign::Center
                            gap=FlexGap::Large
                        >
                            <ActionForm action=update_action>
                                <FieldContextProvider>
                                    <Flex
                                        justify=FlexJustify::Center
                                        align=FlexAlign::Center
                                        gap=FlexGap::Medium
                                        vertical=true
                                    >
                                        <Field label="Nickname" required=true name="nickname">
                                            <Input
                                                value=user.nickname
                                                rules=vec![InputRule::required(true.into())]
                                            />
                                        </Field>

                                        <Field label="Card number" name="card_number">
                                            <Input value=user.card_number.unwrap_or(String::new()) />
                                        </Field>

                                        <input type="hidden" value=user.id.0 name="id" />

                                        <Button
                                            size=ButtonSize::Medium
                                            button_type=ButtonType::Submit
                                            on_click={
                                                let field_context = FieldContextInjection::expect_context();
                                                move |e: ev::MouseEvent| {
                                                    if !field_context.validate() {
                                                        e.prevent_default()
                                                    }
                                                }
                                            }
                                        >
                                            "Update user"
                                        </Button>
                                    </Flex>
                                </FieldContextProvider>
                            </ActionForm>
                            <Flex>
                                <Switch
                                    checked=alternative_color_signal
                                    label="Turn on alternative coloring"
                                />
                            </Flex>
                        </Flex>
                    </div>
                }
                    .into_any()
            }}
        </Suspense>
    }
    .into_any()
}
