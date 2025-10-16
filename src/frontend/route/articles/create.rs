use leptos::{ev, prelude::*};
use thaw::{
    Button, ButtonAppearance, ButtonSize, ButtonType, Field, FieldContextInjection,
    FieldContextProvider, Flex, FlexJustify, Input, InputRule,
};

use crate::{backend::core::behaviour::article_new::CreateArticle, frontend::shared::throw_error};

#[component]
pub fn Create() -> impl IntoView {
    let create_article_action = ServerAction::<CreateArticle>::new();
    Effect::new(move || {
        if let Some(Err(e)) = create_article_action.value().get() {
            throw_error(e);
        }
    });
    view! {
        <Flex justify=FlexJustify::Center>
            <ActionForm action=create_article_action>
                <FieldContextProvider>
                    <Flex vertical=true justify=FlexJustify::SpaceEvenly>
                        <Field label="Name" name="name" required=true>
                            <Input
                                rules=vec![InputRule::required(true.into())]
                                autocomplete="off"
                            />
                        </Field>
                        <Field label="Cost" name="cost" required=true>
                            <Input
                                rules=vec![InputRule::required(true.into())]
                                autocomplete="off"
                            />
                        </Field>
                        <Button
                            button_type=ButtonType::Submit
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Medium
                            on_click={
                                let field_context = FieldContextInjection::expect_context();
                                move |e: ev::MouseEvent| {
                                    if !field_context.validate() {
                                        e.prevent_default();
                                    }
                                }
                            }
                        >
                            "Create Article"
                        </Button>
                    </Flex>
                </FieldContextProvider>
            </ActionForm>
        </Flex>
    }
}
