use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_router::{
    components::Outlet,
    hooks::{use_navigate, use_url},
};
use thaw::{Spinner, Toast, ToastBody, ToastTitle, ToasterInjection};

use crate::{
    backend::core::User,
    frontend::component::{
        figma::{
            button::Button,
            colors::Color,
            icons::{
                bottle::BottleIcon, coins::CoinsIcon, give_money::GiveMoneyIcon,
                logout::LogoutIcon, split_way::SplitWayIcon, time_back::TimeBackIcon,
                user_round::UserRoundIcon,
            },
            spacings::Spacing,
        },
        money::money_diff::MoneyDiff,
    },
};

#[component]
pub fn user_header(user: RwSignal<Option<User>>) -> impl IntoView {
    view! {
        {move || {match user.get() {
            None => view!{
                <Spinner label="Loading user"/>
            }.into_any(),
            Some(user) => {
                view!{
                    <div
                        style:background=Color::BACKGROUND_LIGHT
                    >
                        <div
                            style:max-height=Spacing::px_to_rem(201)
                            style:padding=format!("{} {}", Spacing::XL, Spacing::XXL)
                           class="flex items-center"
                        >
                            <div
                                style:width=Spacing::px_to_rem(310)
                                style:height=Spacing::px_to_rem(81)
                            >

                                <Button
                                    // raw-strings, so helix will highlight it properly
                                    style:width=r#"100%"#
                                    style:height=r#"100%"#

                                    icon=UserRoundIcon
                                    text=user.nickname
                                    on_click=move || {
                                        let navigate = use_navigate();
                                        navigate(&format!("/user/{}/settings", user.id), Default::default())
                                    }
                                />
                            </div>
                            <p
                                style:font-size=Spacing::M
                                style:padding-left=Spacing::M
                            >
                                <MoneyDiff money=user.money />
                            </p>
                            <div
                                style:width=Spacing::px_to_rem(170)
                            />
                            <div

                                style:width=Spacing::px_to_rem(310)
                                style:height=Spacing::px_to_rem(81)
                            >
                                <Button
                                    style:width=r#"100%"#
                                    style:height=r#"100%"#

                                    icon=LogoutIcon
                                    text="Abmelden"
                                    on_click=move || {
                                        let toaster = ToasterInjection::expect_context();

                                        toaster.dispatch_toast(move || {
                                            view!{
                                                <Toast>
                                                    <ToastTitle>{move_tr!("logout-toast-title")}</ToastTitle>
                                                    <ToastBody>{move_tr!("logout-toast-body")}</ToastBody>
                                                </Toast>
                                            }
                                        }, Default::default());

                                        let navigate = use_navigate();
                                        navigate("/", Default::default());
                                    }
                                    background_color=Color::BACKGROUND_LIGHT
                                />
                            </div>
                        </div>
                        <div
                            style:height=Spacing::px_to_rem(151)
                            style:max-height=Spacing::px_to_rem(151)
                            style:width=Spacing::px_to_rem(1080)
                        >
                            <div
                                class="flex place-content-evenly"
                                style:max-height=r#"100%"#
                                style:padding=format!("{} {}", Spacing::M, Spacing::L)
                            >
                                <HeaderIcon icon=BottleIcon text="Kaufen" route=format!("/user/{}", user.id)/>
                                <HeaderIcon icon=CoinsIcon text="Aufladen" route=format!("/user/{}/deposit", user.id)/>
                                <HeaderIcon icon=SplitWayIcon text="Aufteilen" route=format!("/user/{}/split", user.id)/>
                                <HeaderIcon icon=GiveMoneyIcon text="Senden" route=format!("/user/{}/send", user.id)/>
                                <HeaderIcon icon=TimeBackIcon text="Verlauf" route=format!("/user/{}/history", user.id)/>
                            </div>
                        </div>
                    </div>
                    <Outlet/>
                }.into_any()
            }

        }}}
    }
}

#[component]
fn header_icon(
    icon: impl IntoView + 'static,
    #[prop(into)] text: Signal<String>,
    #[prop(into)] route: Signal<String>,
) -> impl IntoView {
    let url = use_url();
    view! {
        <a href=route.get()>
            <div
                class="flex flex-col items-center"
                style:gap=Spacing::XS
            >
                {icon}
                <p
                    style:font-size=Spacing::M
                >
                    {text}
                </p>
                {move || {
                    let url = url.get();
                    if url.path() == route.get() {
                        view!{
                            <hr
                                style:width=r#"150%"#
                                style:border-width=Spacing::px_to_rem(2)
                                style:margin-top=r#"2.3rem"#
                            />
                        }.into_any()
                    } else {().into_any()}
                }}
            </div>
        </a>
    }
}
