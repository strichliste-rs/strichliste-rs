use leptos::prelude::*;
use thaw::{Button, ButtonAppearance};

use crate::{
    frontend::{model::money_args::MoneyArgs, shared::change_money},
    model::{Money, UserPreferences},
};

pub enum ChangeMoneyButtonStyle {
    RED(UserPreferences),
    GREEN(UserPreferences),
}

impl ChangeMoneyButtonStyle {
    pub fn text(&self) -> String {
        match *self {
            ChangeMoneyButtonStyle::RED(settings) => {
                if !settings.alternative_coloring {
                    "text-[#f54963]"
                } else {
                    "text-[#eb99ff]"
                }
            }
            ChangeMoneyButtonStyle::GREEN(settings) => {
                if !settings.alternative_coloring {
                    "text-[#00cc1d]"
                } else {
                    "text-[#9999ff]"
                }
            }
        }
        .to_string()
    }

    pub fn background(&self) -> String {
        match *self {
            ChangeMoneyButtonStyle::RED(settings) => {
                if !settings.alternative_coloring {
                    "bg-[#544052]"
                } else {
                    "bg-[#7a0099]"
                }
            }
            ChangeMoneyButtonStyle::GREEN(settings) => {
                if !settings.alternative_coloring {
                    "bg-[#155949]"
                } else {
                    "bg-[#0000ff]"
                }
            }
        }
        .to_string()
    }
}

#[component]
pub fn ChangeMoneyButton(
    money: i64,
    args: RwSignal<MoneyArgs>,
    preferences: RwSignal<UserPreferences>,
) -> impl IntoView {
    view! {
        {move || {
            let style = if money > 0 {
                ChangeMoneyButtonStyle::GREEN(preferences.get())
            } else {
                ChangeMoneyButtonStyle::RED(preferences.get())
            };
            let class = format!("p-5 {} {}", style.text(), style.background());

            view! {
                <Button
                    appearance=ButtonAppearance::Primary
                    class=class
                    on_click=move |_| change_money(money.into(), args)
                >
                    {Money::format_eur_diff_value(money)}
                </Button>
            }
        }}
    }
}
