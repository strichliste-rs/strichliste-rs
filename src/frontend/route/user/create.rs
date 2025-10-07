use leptos::{ev, prelude::*};
use thaw::{
    Button, ButtonSize, ButtonType, Field, FieldContextInjection, FieldContextProvider,
    FieldOrientation, Flex, FlexAlign, FlexGap, FlexJustify, Input, InputRule, InputSize,
};

use crate::{backend::core::behaviour::user_create::CreateUser, frontend::shared::throw_error};

#[component]
pub fn Create() -> impl IntoView {
    let create_user_action = ServerAction::<CreateUser>::new();

    Effect::new(move || {
        if let Some(Err(e)) = create_user_action.value().get() {
            let msg = match e {
                ServerFnError::ServerError(msg) => msg,
                _ => e.to_string(),
            };
            throw_error(format!("Failed to create user: {msg}"))
        }
    });

    view! {
        <Flex justify=FlexJustify::Center>
            <Flex align=FlexAlign::Center vertical=true>
                <div class="pt-4">
                    <ActionForm action=create_user_action>
                        <FieldContextProvider>
                            <Flex align=FlexAlign::Center vertical=true justify=FlexJustify::Center gap=FlexGap::Medium>
                                <Field label="Username" name="username" class="text-center" required=true orientation=FieldOrientation::Vertical>
                                    <Input rules=vec![InputRule::required(true.into())] autocomplete="off" size=InputSize::Medium name="username"/>
                                </Field>
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
                                    "Create User"
                                </Button>
                            </Flex>
                        </FieldContextProvider>
                    </ActionForm>
                </div>
            </Flex>
        </Flex>
    }
}
