use std::str::FromStr;

use leptos::{leptos_dom::logging::console_log, prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;
use thaw::{Button, Input, Spinner, ToasterInjection};

use crate::{
    backend::core::behaviour::article_get_by_barcode::get_article_by_barcode,
    frontend::{
        component::{
            article::buy_article::BuyArticle,
            change_money_button::ChangeMoneyButton,
            icon::{SendMoneyIcon, SettingsIcon},
            return_to::ReturnTo,
            scan_input::ScanInput,
            transaction::ShowTransactions,
        },
        model::{
            cachinglayer::CachingLayer,
            frontend_store::{FrontendStoreStoreFields, FrontendStoreType},
            money_args::MoneyArgs,
        },
        shared::{buy_article, on_custom_money_button_click, throw_error, throw_error_none_view},
    },
    model::{Transaction, UserId},
};

pub const RETURN_TO_MAIN_VIEW_TIMEOUT_SEC: u64 = 15;

#[component]
pub fn ShowUser() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = match user_id_string.parse::<i64>() {
        Ok(id) => UserId(id),
        Err(e) => {
            return throw_error_none_view(format!("Failed to convert id to a number: {e}"));
        }
    };

    let store = expect_context::<FrontendStoreType>();

    let cache = store.cachinglayer().get_untracked();

    let entry = CachingLayer::get_user(cache, user_id);

    view! {
        <ReturnTo after=RETURN_TO_MAIN_VIEW_TIMEOUT_SEC route="/" />
        <div>
            {move || {
                let (is_fetching, value) = {
                    let entry = entry.read();
                    (entry.is_fetching, entry.value)
                };
                if is_fetching.get() && value.get().is_none() {
                    return view! { <Spinner label="Loading user!" /> }.into_any();
                }
                let user = match value.get() {
                    Some(user) => user,
                    None => {
                        return throw_error_none_view(
                            format!("No user with the id {} has been found!", user_id.0),
                        );
                    }
                };
                let money_signal = RwSignal::new(user.money);
                let transactions = RwSignal::new(Vec::<Transaction>::new());
                let money_args = RwSignal::new(MoneyArgs {
                    user_id,
                    money: money_signal,
                    transactions,
                });
                let custom_money_change = RwSignal::new(String::from_str("0.00").unwrap());
                let custom_money_is_focused = RwSignal::new(false);
                let toaster = ToasterInjection::expect_context();

                view! {
                    <ScanInput
                        ignore_input_signals=vec![custom_money_is_focused]
                        callback=move |scan_input| {
                            spawn_local(async move {
                                console_log(&format!("Input {scan_input}"));
                                let article = get_article_by_barcode(scan_input.clone()).await;
                                let article = match article {
                                    Ok(value) => value,
                                    Err(e) => {
                                        throw_error(
                                            format!("Failed to fetch article from server: {e}"),
                                        );
                                        return;
                                    }
                                };
                                match article {
                                    None => {
                                        throw_error(
                                            format!(
                                                "No article could be found with barcode '{scan_input}'",
                                            ),
                                        );
                                    }
                                    Some(value) => {
                                        buy_article(value.id, value.cost, money_args, toaster);
                                    }
                                }
                            });
                        }
                    />
                    <div class="grid grid-cols-2">
                        <div class="pt-5">
                            // left side (show user statistics)
                            <div class="grid grid-cols-3">
                                <div class="col-span-2">
                                    <div class="flex place-content-evenly flex-col gap-[1.5em]">
                                        <p class="text-center text-white text-[2em]">
                                            {user.nickname.clone()}
                                        </p>
                                        <p
                                            class="text-center text-[2em]"
                                            class=(
                                                "text-red-500",
                                                move || (money_signal.get()).value < 0,
                                            )
                                            class=(
                                                "text-green-500",
                                                move || (money_signal.get()).value >= 0,
                                            )
                                        >

                                            {move || (money_signal.get()).format_eur_diff()}
                                        </p>
                                    </div>
                                </div>
                                <div class="col-span-1">
                                    <div class="flex justify-evenly">
                                        <a
                                            href=format!("/user/{}/settings", user_id)
                                            class="text-white pt-[5px] flex flex-col items-center"
                                        >
                                            <SettingsIcon />
                                            <p class="text-center">"Settings"</p>
                                        </a>
                                        <a
                                            href=format!("/user/{}/send_money", user_id)
                                            class="text-white w-[3rem] flex flex-col items-center"
                                        >
                                            <SendMoneyIcon />
                                            <p class="text-center">"Send money"</p>
                                        </a>
                                    </div>
                                </div>
                            </div>
                            <BuyArticle args=money_args />
                        </div>
                        <div>
                            // right side (put in money)
                            <div class="flex flex-col gap-3 bg-gray-500 p-3 rounded-[10px]">
                                <div class="grid grid-cols-3 gap-5 rounded-[10px]">
                                    <ChangeMoneyButton money=50 args=money_args />
                                    <ChangeMoneyButton money=100 args=money_args />
                                    <ChangeMoneyButton money=200 args=money_args />
                                    <ChangeMoneyButton money=500 args=money_args />
                                    <ChangeMoneyButton money=1000 args=money_args />
                                    <ChangeMoneyButton money=2000 args=money_args />
                                    <ChangeMoneyButton money=5000 args=money_args />
                                </div>
                                <div class="grid grid-cols-3 gap-3">
                                    <Button
                                        class="bg-red-400 text-white rounded-full p-5"
                                        on_click=move |_| on_custom_money_button_click(
                                            false,
                                            custom_money_change,
                                            money_args,
                                        )
                                    >
                                        <div class="pad-5 text-center">"-"</div>
                                    </Button>
                                    <Input
                                        // class="text-center"
                                        placeholder="Euros"
                                        // cannot autofocus, since we might want to scan a barcode
                                        autofocus=false
                                        value=custom_money_change
                                        on:focus=move |_| { custom_money_is_focused.set(true) }
                                        on:blur=move |_| { custom_money_is_focused.set(false) }
                                    >// has some stupid border for some reason
                                    // <InputSuffix slot>"€"</InputSuffix>
                                    </Input>
                                    <Button
                                        class="bg-emerald-600 text-white rounded-full p-5"
                                        on_click=move |_| on_custom_money_button_click(
                                            true,
                                            custom_money_change,
                                            money_args,
                                        )
                                    >
                                        <div class="pad-5 text-center">"+"</div>
                                    </Button>
                                </div>
                                <div class="grid grid-cols-3 gap-5 rounded-[10px]">
                                    <ChangeMoneyButton money=-50 args=money_args />
                                    <ChangeMoneyButton money=-100 args=money_args />
                                    <ChangeMoneyButton money=-200 args=money_args />
                                    <ChangeMoneyButton money=-500 args=money_args />
                                    <ChangeMoneyButton money=-1000 args=money_args />
                                    <ChangeMoneyButton money=-2000 args=money_args />
                                    <ChangeMoneyButton money=-5000 args=money_args />

                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="pt-5">
                        <ShowTransactions arguments=money_args />
                    </div>
                }
                    .into_any()
            }}
        </div>
    }
    .into_any()
}
