use leptos::prelude::*;
use thaw::Button as ThawButton;

use crate::frontend::component::figma::{colors::Color, spacings::Spacing};

#[component]
pub fn button<F, I>(
    on_click: F,
    icon: I,
    #[prop(into)] text: Signal<String>,
    #[prop(optional)] background_color: Option<&'static str>,
    #[prop(optional)] border_color: Option<&'static str>,
) -> impl IntoView
where
    F: Fn() + 'static + Send + Sync,
    I: IntoView + 'static,
{
    view! {
        <ThawButton
            style:background=background_color.unwrap_or(Color::PRIMARY)
            style:border-radius=Spacing::XS
            style:border=border_color.unwrap_or("transparent")
            on_click=move |_| {
                on_click()
            }
        >
            <div
                class="flex items-center gap-2"
                style:padding="0 10px"
            >

                <div
                    class="flex justify-center items-center h-[2.5rem]"
                    style:width=Spacing::L
                >
                    {icon}
                </div>
                <p class="font-[400]" style:font-size=Spacing::M>
                    {text}
                </p>

            </div>
        </ThawButton>
    }
}
