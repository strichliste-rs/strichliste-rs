pub const PREFIX_FILTER_NON_ALPHABETIC_VALUE: char = '!';

use leptos::prelude::*;
use thaw::Flex;

use crate::frontend::component::{colors::Colors, icon::UserViewArrowLeft};

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div>
            <Flex
                vertical=false
                gap=thaw::FlexGap::Medium
            >
                <UserViewArrowLeft/>
                <p>"Anmelden"</p>
            </Flex>
        </div>
    }
}
