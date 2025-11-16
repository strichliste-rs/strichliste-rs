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
            style:background-color=Color::BackgroundLight.get()
            style:padding=Spacing::XXL.get()

            style:box-shadow="0 10px 8px black"
        >
            <Flex align=FlexAlign::FlexStart vertical=true style:gap=Spacing::L.get()>
                <Flex vertical=false align=FlexAlign::Center>

                    <div
                        class="w-[100px] h-[80px] flex justify-center items-center"
                        style:border-radius=Spacing::XS.get()
                        style:padding-top=Spacing::S.get()
                        style:padding-right=Spacing::M.get()
                        style:padding-bottom=Spacing::S.get()
                        style:padding-left=Spacing::M.get()
                        style:gap=Spacing::S.get()
                    >
                        <ArrowLeftIcon />
                    </div>
                    <p
                        class="font-bold max-w-[205px] max-h-[54px]"
                        style:color=Color::TextMain.get()
                        style:font-size=Spacing::L.get()
                    >
                        {move_tr!("login")}
                    </p>
                </Flex>
                <Flex style:gap=Spacing::M.get() style:max-height="81px">
                    // neuer account
                    <Button
                        style:background=Color::Primary.get()
                        style:border-radius=Spacing::XS.get()
                    >
                        <Flex
                            align=FlexAlign::Center
                            justify=FlexJustify::Center
                            gap=thaw::FlexGap::Medium
                            style:padding-top=Spacing::S.get()
                            style:padding-right=Spacing::M.get()
                            style:padding-bottom=Spacing::S.get()
                            style:padding-left=Spacing::M.get()
                        >

                            <div
                                class="flex justify-center items-center h-[40px]"
                                style:width=Spacing::L.get()
                            >
                                <AddIcon />
                            </div>
                            <p class="font-[400]" style:font-size=Spacing::M.get()>
                                {move_tr!("new-account")}
                            </p>

                        </Flex>
                    </Button>
                    // removes the border of selected inputs
                    <Style>
                        r#".thaw-input__input {
                            outline: none
                        }"#
                    </Style>
                    <Input
                        input_type=InputType::Text
                        placeholder=move_tr!("search")

                        style:font-size=Spacing::M.get()
                        style:border-radius=Spacing::XS.get()
                    >
                        <InputPrefix slot>
                            <Icon
                                icon=icondata::LuSearch
                                style:color="white"
                                style:border="none"
                                style:height="50px"
                                style:width="50px"
                                style:padding-left="10px"
                                style:padding-right="10px"
                            />
                        </InputPrefix>
                    </Input>
                    <Flex
                        align=FlexAlign::Center
                        style:padding=format!("{} {}", Spacing::S.get(), Spacing::M.get())
                    >
                        <RotateLeftIcon />
                    </Flex>
                </Flex>
            </Flex>
        </div>
    }
}
