use leptos::prelude::*;

use leptos_fluent::move_tr;
use leptos_meta::Style;
use thaw::{Button, Flex, FlexAlign, FlexJustify, Icon, Input, InputPrefix, InputType};

use crate::frontend::component::figma::{
    colors::Color,
    icons::{add::AddIcon, arrow_left::ArrowLeftIcon, rotate_left::RotateLeftIcon},
    spacings::Spacing,
};

#[component]
pub fn HomeHeader() -> impl IntoView {
    view! {
        <div
            style:background-color=Color::BACKGROUND_LIGHT
            style:padding=Spacing::XXL

            style:box-shadow="0 10px 8px #00000040"
        >
            <Flex align=FlexAlign::FlexStart vertical=true style:gap=Spacing::L>
                <Flex vertical=false align=FlexAlign::Center>

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
                        style:background=Color::PRIMARY
                        style:border-radius=Spacing::XS
                    >
                        <Flex
                            align=FlexAlign::Center
                            justify=FlexJustify::Center
                            gap=thaw::FlexGap::Medium
                            style:padding="0 10px"
                        >

                            <div
                                class="flex justify-center items-center h-[2.5rem]"
                                style:width=Spacing::L
                            >
                                <AddIcon />
                            </div>
                            <p class="font-[400]" style:font-size=Spacing::M>
                                {move_tr!("new-account")}
                            </p>

                        </Flex>
                    </Button>
                    // removes the border of selected inputs
                    // and moves the placeholder more to the right
                    <Style>
                        {format!(r#"
                            .thaw-input__input {{
                                outline: none;
                            }}
                        
                            .thaw-input--prefix {{
                                .thaw-input__input {{
                                    padding-left: {};
                                }}
                            }}
                        "#, Spacing::XS)}
                    </Style>
                    <Input
                        input_type=InputType::Text
                        placeholder=move_tr!("search")

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
