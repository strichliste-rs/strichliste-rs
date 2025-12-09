use leptos::{prelude::*, reactive::spawn_local};

use leptos_fluent::move_tr;
use leptos_meta::Style;
use thaw::{Flex, FlexAlign, Icon, Input, InputPrefix, InputType};

use crate::{
    backend::core::behaviour::user_create::create_user,
    frontend::{
        component::figma::{
            button::Button,
            colors::Color,
            dialog::{InputDialog, InputDialogType},
            icons::{add::AddIcon, arrow_left::ArrowLeftIcon, rotate_left::RotateLeftIcon},
            spacings::Spacing,
        },
        shared::throw_error,
    },
};

#[component]
pub fn HomeHeader(user_filter: RwSignal<String>) -> impl IntoView {
    let create_new_user_trigger = RwSignal::new(false);
    view! {
        <InputDialog
            title=move_tr!("dialog-newAccount-title")
            description=move_tr!("dialog-newAccount-description")
            ok_button_text=move_tr!("dialog-newAccount-okButton")
            open_signal=create_new_user_trigger
            input_dialog_type=InputDialogType::Text
            on_ok=move |input| {
                spawn_local(async move {
                    if let Err(e) = create_user(input).await {
                        throw_error(format!("Failed to create user: {}", e));
                    }
                });
            }
        />
        <div
            style:background-color=Color::BACKGROUND_LIGHT
            style:padding=Spacing::XXL

            style:box-shadow="0 10px 8px #00000040"
        >
            <Flex align=FlexAlign::FlexStart vertical=true style:gap=Spacing::L>
                <Flex align=FlexAlign::Center>

                    <div
                        class="w-[6.25rem] h-[5rem] flex justify-center items-center"
                        style:border-radius=Spacing::XS
                        style:padding-top=Spacing::S
                        style:padding-right=Spacing::M
                        style:padding-bottom=Spacing::S
                        style:padding-left=Spacing::M
                        style:gap=Spacing::S
                    >
                        <ArrowLeftIcon />
                    </div>
                    <p
                        class="font-bold max-w-[205px] max-h-[54px]"
                        style:color=Color::TEXT_MAIN
                        style:font-size=Spacing::L
                    >
                        {move_tr!("login")}
                    </p>
                </Flex>
                <Flex style:gap=Spacing::M style:max-height="81px">
                    // neuer account
                    <Button
                        icon=AddIcon
                        text=move_tr!("new-account")
                        on_click=move || {
                            create_new_user_trigger.set(true)
                        }
                    />
                    // removes the border of selected inputs
                    // and moves the placeholder more to the right
                    <Style blocking="true">
                        {format!(
                            r#"
                            .thaw-input__input {{
                                outline: none;
                            }}
                        
                            .thaw-input--prefix {{
                                .thaw-input__input {{
                                    padding-left: {};
                                }}
                            }}
                        "#,
                            Spacing::XS,
                        )}
                    </Style>
                    <Input
                        input_type=InputType::Text
                        placeholder=move_tr!("search")
                        value=user_filter

                        style:font-size=Spacing::M
                        style:border-radius=Spacing::XS
                        style:max-width="430px"
                    >
                        <InputPrefix slot>
                            <Icon
                                icon=icondata::LuSearch
                                style:color="white"
                                style:border="none"
                                style:height="3.125rem"
                                style:width="3.125rem"
                                style:padding-left=Spacing::px_to_rem(10)
                                style:padding-right=Spacing::px_to_rem(10)
                            />
                        </InputPrefix>
                    </Input>
                    <Flex
                        align=FlexAlign::Center
                        style:padding=format!("{} {}", Spacing::S, Spacing::M)
                        on:click=move |_| {user_filter.set(String::new())}
                    >
                        <RotateLeftIcon />
                    </Flex>
                </Flex>
            </Flex>
        </div>
        // in order for the box-shadow to show
        <div style:padding-top="4%"></div>
    }
}
