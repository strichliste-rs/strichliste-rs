pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';

use leptos::prelude::*;
use thaw::{Button, Flex, FlexAlign, FlexJustify};

use crate::frontend::component::{
    figma::{colors::Color, spacings::Spacing},
    icon::{user_view_arrow_left::IconUserViewArrowLeft, user_view_plus::IconUserViewPlus},
};

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div style:background-color=Color::BackgroundLight.get() style:padding=Spacing::XXL.get()>
            <Flex gap=thaw::FlexGap::Medium align=FlexAlign::FlexStart vertical=true>
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
                        <IconUserViewArrowLeft />
                    </div>
                    <p
                        class="font-bold"
                        style:color=Color::TextMain.get()
                        style:font-size=Spacing::L.get()
                    >
                        "Anmelden"
                    </p>
                </Flex>
                <div>
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
                                <IconUserViewPlus />
                            </div>
                            <p class="font-[400]" style:font-size=Spacing::M.get()>
                                "Neuer Account"
                            </p>

                        </Flex>
                    </Button>
                </div>
            </Flex>
        </div>
    }
}
