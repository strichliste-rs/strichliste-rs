use leptos::{ev, prelude::*, reactive::spawn_local};
use leptos_router::hooks::{use_navigate, use_query_map};
use thaw::{
    Button, ButtonType, ComponentRef, Field, FieldContextInjection, FieldContextProvider, Flex,
    FlexGap, Input, InputRef, InputRule, Popover, PopoverTrigger, PopoverTriggerType, Toast,
    ToastBody, ToastTitle, ToasterInjection,
};

use crate::{
    backend::core::{
        behaviour::{user_create::CreateUser, user_get_by_card_number::get_user_by_barcode},
        User,
    },
    frontend::{
        component::{return_to::ReturnTo, scan_input::ScanInput, user::ShowUsers},
        shared::throw_error,
    },
};

pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';
const PREFIX_FILTER_CLEAR_TIMEOUT_SEC: u64 = 15;
const PREFIX_FILTER_NAME: &str = "filter";

#[component]
pub fn View() -> impl IntoView {
    let create_user_action = ServerAction::<CreateUser>::new();

    Effect::new(move || {
        if let Some(Err(e)) = create_user_action.value().get() {
            let msg = match e {
                ServerFnError::ServerError(msg) => msg,
                _ => e.to_string(),
            };
            throw_error(format!("Failed to add user: {msg}"));
        }
    });

    let found_user_signal: RwSignal<Option<User>> = RwSignal::new(None);

    Effect::new(move || {
        if let Some(user) = found_user_signal.get() {
            let navigate = use_navigate();
            navigate(&format!("/user/{}", user.id), Default::default());
        }
    });

    let toaster = ToasterInjection::expect_context();

    let ignore_scan_input_signal = RwSignal::new(false);
    let input_ref = ComponentRef::<InputRef>::new();

    let querys = use_query_map();
    let filter_prefix = Signal::derive(move || match querys.read().get(PREFIX_FILTER_NAME) {
        Some(s) => s.chars().nth(0),
        None => None,
    });

    Effect::new(move || {
        if let Some(input) = input_ref.get() {
            input.focus();
        }
    });

    view! {
        {move || filter_prefix.get().map(|_| view!{ < ReturnTo route="/" after=PREFIX_FILTER_CLEAR_TIMEOUT_SEC />})}
        <div class="grid grid-cols-10 gap-10 py-10 h-screen">
            <div class="col-span-1 pl-5 justify-self-center">
                <div class="flex flex-col">
                    <Popover
                        trigger_type=PopoverTriggerType::Click
                        on_open=move || { ignore_scan_input_signal.set(true) }
                        on_close=move || { ignore_scan_input_signal.set(false) }
                    >
                        <PopoverTrigger slot>
                            // joinked from: https://gist.github.com/ibelick/0c92c1aba54c2f7e8b3a7381426ed029
                            <button class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-gray-50 text-black drop-shadow-sm transition-colors duration-150 hover:bg-gray-200">
                                "+"
                            </button>
                        </PopoverTrigger>
                        <ActionForm action=create_user_action prop:autocomplete="off">
                            <FieldContextProvider>
                                <Flex gap=FlexGap::Medium>
                                    <Field name="username">
                                        <Input
                                            rules=vec![InputRule::required(true.into())]
                                            comp_ref=input_ref
                                        />
                                    </Field>
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
                                        "Create User"
                                    </Button>
                                </Flex>
                            </FieldContextProvider>
                        </ActionForm>
                    </Popover>
                    <div class="flex flex-full flex-col mt-4">
                        <a
                            class="text-center mb-1.5"
                            href=format!(
                                "/?{}={}",
                                PREFIX_FILTER_NAME,
                                PREFIX_FILTER_NON_ALPHABETIC_VALUE,
                            )
                        >
                            #
                        </a>
                        {('A'..='Z')
                            .map(|letter| {
                                view! {
                                    <a
                                        class="text-center mb-1.5"
                                        href=format!(
                                            "/?{}={}",
                                            PREFIX_FILTER_NAME,
                                            letter.to_ascii_lowercase(),
                                        )
                                    >
                                        {letter}
                                    </a>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
            <ScanInput
                ignore_input_signals=vec![ignore_scan_input_signal]
                callback=move |input_string| {
                    spawn_local(async move {
                        let user = match get_user_by_barcode(input_string.clone()).await {
                            Ok(user) => user,
                            Err(err) => {
                                throw_error(format!("Failed to fetch user by barcode: {}", err));
                                return;
                            }
                        };
                        match user {
                            Some(user) => found_user_signal.set(Some(user)),
                            None => {
                                toaster
                                    .dispatch_toast(
                                        move || {
                                            view! {
                                                <Toast>
                                                    <ToastTitle>"Failed to find user"</ToastTitle>
                                                    <ToastBody>
                                                        {format!(
                                                            "There is no user with barcode \"{input_string}\"",
                                                        )}
                                                    </ToastBody>
                                                </Toast>
                                            }
                                        },
                                        Default::default(),
                                    );
                            }
                        };
                    });
                }
            />
            <div class="col-span-9 pr-7">
                <ShowUsers filter_prefix=filter_prefix />
            </div>
        </div>
    }
}
