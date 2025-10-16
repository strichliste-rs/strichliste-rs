use crate::{
    backend::core::behaviour::{update_user::UpdateUser, user_get::get_user},
    frontend::{
        component::return_to::ReturnTo, route::user::RETURN_TO_MAIN_VIEW_TIMEOUT_SEC,
        shared::throw_error_none_view,
    },
    model::UserId,
};
use leptos::{
    ev::{self},
    prelude::*,
};
use leptos_router::hooks::use_params_map;
use thaw::{
    Button, ButtonSize, ButtonType, Field, FieldContextInjection, FieldContextProvider, Flex,
    FlexAlign, FlexGap, FlexJustify, Input, InputRule,
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
            view! { <p class="text-white text-center pt-5">"Loading User..."</p> }
        }>
            {move || {
                let user = match user_resource.get() {
                    Some(user) => user,
                    None => {
                        return ().into_any();
                    }
                };
                let user = match user {
                    Ok(user) => user,
                    Err(err) => {
                        return throw_error_none_view(
                            format!("Failed to fetch user because: {err}"),
                        );
                    }
                };
                let user = match user {
                    Some(user) => user,
                    None => {
                        return throw_error_none_view(
                            format!("No user with the id {} has been found!", user_id.0),
                        );
                    }
                };

                view! {
                    {move || match update_action.value().get() {
                        Some(Err(e)) => {
                            let msg = match e {
                                ServerFnError::ServerError(msg) => msg,
                                _ => e.to_string(),
                            };
                            throw_error_none_view(format!("Failed to update user: {msg}"))
                        }
                        _ => ().into_any(),
                    }}
                    <ActionForm action=update_action>
                        <FieldContextProvider>
                            <div class="pt-5">
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
                            </div>
                        </FieldContextProvider>
                    </ActionForm>
                }
                    .into_any()
            }}
        </Suspense>
    }
    .into_any()
}
