use leptos::{ev, prelude::*};
use leptos_router::hooks::use_params_map;
use thaw::{
    AutoComplete, AutoCompleteOption, Button, ButtonType, Field, FieldContextInjection,
    FieldContextProvider, Flex, FlexAlign, FlexGap, FlexJustify, Input, Label,
};

use crate::{
    backend::core::behaviour::{
        send_money::SendMoney, user_get::get_user, user_get_all::get_all_users,
    },
    frontend::shared::{throw_error, throw_error_none_view},
    model::UserId,
};

const USER_SEARCH_LIMIT: usize = 10;

#[component]
pub fn Show() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(value) => value,
        Err(_e) => {
            return throw_error_none_view(format!(
                "Failed to convert id: {user_id_string} to a number!"
            ));
        }
    };

    let user_id = UserId(user_id);

    let user_resource = OnceResource::new(get_user(user_id));
    let all_users_resource = OnceResource::new(get_all_users());

    let send_money_action = ServerAction::<SendMoney>::new();

    view! {
        <Suspense fallback=move || {
            view! { <p class="text-white text-center">"Loading user"</p> }
        }>
            {move || {
                user_resource
                    .get()
                    .map(|user| {
                        let user = match user {
                            Ok(value) => value,
                            Err(e) => {
                                return throw_error_none_view(format!("Failed to fetch user: {e}"));
                            }
                        };
                        let user = match user {
                            Some(value) => value,
                            None => {
                                return throw_error_none_view(
                                    format!("No such user with id '{}' exists!", user_id.0),
                                );
                            }
                        };
                        let all_users = match all_users_resource.get() {
                            Some(Ok(value)) => RwSignal::new(value),
                            _ => {
                                return ().into_any();
                            }
                        };
                        let selected_user_input = RwSignal::new(String::new());
                        let money_amount_input = RwSignal::new(String::new());
                        let options = Memo::<
                            Vec<String>,
                        >::new(move |_| {
                            all_users
                                .get()
                                .iter()
                                .filter(|elem| elem.id != user.id)
                                .filter(|elem| {
                                    elem
                                        .nickname
                                        .to_lowercase()
                                        .contains(&selected_user_input.get().to_lowercase())
                                })
                                .map(|elem| elem.nickname.clone())
                                .take(USER_SEARCH_LIMIT)
                                .collect()
                        });
                        Effect::new(move || {
                            if let Some(Err(e)) = send_money_action.value().get() {
                                throw_error(format!("Failed to send money: {e}"));
                            }
                        });

                        view! {
                            <div class="pt-5">
                                <Flex
                                    justify=FlexJustify::Center
                                    align=FlexAlign::Center
                                    vertical=true
                                >
                                    <Label>
                                        "Hello "{user.nickname}", who do you want to send money to?"
                                    </Label>
                                    <ActionForm action=send_money_action prop:autocomplete="off">
                                        <FieldContextProvider>
                                            <Flex
                                                vertical=true
                                                align=FlexAlign::Center
                                                gap=FlexGap::Medium
                                            >
                                                <Field name="to_user" label="Receiver" required=true>
                                                    <AutoComplete
                                                        value=selected_user_input
                                                        placeholder="Receiving User"
                                                    >
                                                        <For
                                                            each=move || options.get()
                                                            key=|option| option.clone()
                                                            let:option
                                                        >
                                                            <AutoCompleteOption value=option
                                                                .clone()>{option}</AutoCompleteOption>
                                                        </For>
                                                    </AutoComplete>
                                                </Field>
                                                <Field name="amount" label="Amount" required=true>
                                                    <Input value=money_amount_input />
                                                </Field>

                                                <input type="hidden" name="user_id" value=user.id.0 />

                                                <Button
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
                                                    "Send money"
                                                </Button>
                                            </Flex>
                                        </FieldContextProvider>
                                    </ActionForm>
                                </Flex>

                            </div>
                        }
                            .into_any()
                    })
            }}
        </Suspense>
    }.into_any()
}
