use std::str::FromStr;

use leptos::{leptos_dom::logging::console_log, prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_params_map;
use thaw::{Button, Input, Spinner, ToasterInjection};

use crate::{
    backend::core::behaviour::article_get_by_barcode::get_article_by_barcode,
    frontend::{
        component::{
            article::buy_article::BuyArticle, change_money_button::ChangeMoneyButton,
            icon::SettingsIcon, return_to::ReturnTo, scan_input::ScanInput,
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
                                            {SettingsIcon()}
                                            <p class="text-center">"Settings"</p>
                                        </a>
                                        <a
                                            href=format!("/user/{}/send_money", user_id)
                                            class="text-white w-[3rem] flex flex-col items-center"
                                        >
                                            <svg
                                                viewBox="0 0 24 24"
                                                fill="none"
                                                xmlns="http://www.w3.org/2000/svg"
                                            >
                                                <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
                                                <g
                                                    id="SVGRepo_tracerCarrier"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                ></g>
                                                <g id="SVGRepo_iconCarrier">
                                                    <path
                                                        d="M12 3V9M12 3L9.5 5.5M12 3L14.5 5.5M5.82333 9.00037C6.2383 9.36683 6.5 9.90285 6.5 10.5C6.5 11.6046 5.60457 12.5 4.5 12.5C3.90285 12.5 3.36683 12.2383 3.00037 11.8233M5.82333 9.00037C5.94144 9 6.06676 9 6.2 9H8M5.82333 9.00037C4.94852 9.00308 4.46895 9.02593 4.09202 9.21799C3.71569 9.40973 3.40973 9.71569 3.21799 10.092C3.02593 10.469 3.00308 10.9485 3.00037 11.8233M3.00037 11.8233C3 11.9414 3 12.0668 3 12.2V17.8C3 17.9332 3 18.0586 3.00037 18.1767M3.00037 18.1767C3.36683 17.7617 3.90285 17.5 4.5 17.5C5.60457 17.5 6.5 18.3954 6.5 19.5C6.5 20.0971 6.2383 20.6332 5.82333 20.9996M3.00037 18.1767C3.00308 19.0515 3.02593 19.5311 3.21799 19.908C3.40973 20.2843 3.71569 20.5903 4.09202 20.782C4.46895 20.9741 4.94852 20.9969 5.82333 20.9996M5.82333 20.9996C5.94144 21 6.06676 21 6.2 21H17.8C17.9332 21 18.0586 21 18.1767 20.9996M21 18.1771C20.6335 17.7619 20.0973 17.5 19.5 17.5C18.3954 17.5 17.5 18.3954 17.5 19.5C17.5 20.0971 17.7617 20.6332 18.1767 20.9996M21 18.1771C21.0004 18.0589 21 17.9334 21 17.8V12.2C21 12.0668 21 11.9414 20.9996 11.8233M21 18.1771C20.9973 19.0516 20.974 19.5311 20.782 19.908C20.5903 20.2843 20.2843 20.5903 19.908 20.782C19.5311 20.9741 19.0515 20.9969 18.1767 20.9996M20.9996 11.8233C20.6332 12.2383 20.0971 12.5 19.5 12.5C18.3954 12.5 17.5 11.6046 17.5 10.5C17.5 9.90285 17.7617 9.36683 18.1767 9.00037M20.9996 11.8233C20.9969 10.9485 20.9741 10.469 20.782 10.092C20.5903 9.71569 20.2843 9.40973 19.908 9.21799C19.5311 9.02593 19.0515 9.00308 18.1767 9.00037M18.1767 9.00037C18.0586 9 17.9332 9 17.8 9H16M14 15C14 16.1046 13.1046 17 12 17C10.8954 17 10 16.1046 10 15C10 13.8954 10.8954 13 12 13C13.1046 13 14 13.8954 14 15Z"
                                                        stroke="#ffffff"
                                                        stroke-width="2"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                    ></path>
                                                </g>
                                            </svg>
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
