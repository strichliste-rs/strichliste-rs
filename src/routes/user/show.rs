use std::rc::Rc;

use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use tracing::error;

use crate::
    models::{Transaction, TransactionType, User}
;

#[derive(Debug, Clone)]
pub struct MoneyArgs {
    user_id: i64,
    money_read: ReadSignal<i64>,
    money_write: WriteSignal<i64>,
    error_write: WriteSignal<String>,
}

#[server]
pub async fn get_user(id: i64) -> Result<Option<User>, ServerFnError> {
    use crate::backend::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let response_opts: ResponseOptions = expect_context();

    let user = User::get_by_id(&*state.db.lock().await, id).await;

    if user.is_err() {
        let err = user.err().unwrap();
        error!("Failed to fetch user: {}", err);
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new(err));
    }

    let user = user.unwrap();

    Ok(user)
}

#[server]
pub async fn create_transaction(user_id: i64, money_diff: i64, transaction_type: TransactionType) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();

    if transaction_type == TransactionType::UNKNOWN {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Bad transaction type given!"));
    }

    let user = get_user(user_id).await?;

    if user.is_none() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new(&format!(
            "No user found with id {}",
            user_id
        )));
    }

    let mut transaction = Transaction::new();

    transaction.t_type = transaction_type;
    transaction.money = money_diff;
    transaction.user_id = user_id;

    let result = transaction.add_to_db(&*state.db.lock().await).await;

    if result.is_err() {
        let err = result.err().unwrap();
        error!("{err}");
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new(err));
    }

    let mut user = user.unwrap();

    user.money += money_diff;

    let result = user.update_money(&*state.db.lock().await).await;

    if result.is_err() {
        let err = result.err().unwrap();
        error!("{err}");
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new(err));
    }

    Ok(())
}

#[component]
pub fn ShowUser() -> impl IntoView {
    let params = use_params_map();
    let user_id_string = params.read_untracked().get("id").unwrap_or_default();

    let user_id = user_id_string.parse::<i64>();

    if user_id.is_err() {
        return view! {
            <p class="text-red-500">"Failed to convert id to a number!"</p>
        }
        .into_any();
    }

    let user_id = user_id.unwrap();

    let user_resource = OnceResource::new(get_user(user_id));

    let (error_read, error_write) = signal(String::new());

    return view! {
        {
            move || {
                let error = error_read.get();

                if error.len() != 0 {
                    view! {
                        
                        <div>
                            <p class="text-white bg-red-400 p-5 text-center">"An error has occurred: "{error}</p>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }

            }
        }
        { move || {

            view!{
                <Suspense
                    fallback=move || view!{<p class="text-white text-center pt-5">"Loading user..."</p>}
                >
                <div>
                    {
                        move || {
                            let user = user_resource.get();

                            if user.is_none() {
                                return view!{
                                    <p class="text-red-500">"Failed to fetch user"</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            if user.is_err(){
                                let err = user.err().unwrap().to_string();
                                return view!{
                                    <p class="text-red-500">"Failed to fetch user because: "{err}</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            if user.is_none(){
                                return view! {
                                    <p class="text-red-500">"No user with the id "{user_id}" has been found!"</p>
                                }.into_any();
                            }

                            let user = user.unwrap();

                            let (read_money, write_money) = signal(user.money);

                            let m_args = MoneyArgs {
                                user_id: user_id,
                                money_read: read_money,
                                money_write: write_money,
                                error_write: error_write,
                            };

                            let args1 = m_args.clone();
                            let args2 = m_args.clone();
                            
                            let args = Rc::new(m_args);

                            let custom_money_change = RwSignal::new(String::new());

                            view!{
                                <div class="grid grid-cols-2">
                                    <div class="pt-5">
                                        // left side (show user statistics)
                                        <div class="grid grid-cols-4">
                                            <div class="col-span-3">
                                                <p class="text-center text-white text-[2em]">{user.nickname.clone()}</p>
                                                <p class="text-center text-[2em]"
                                                    class=("text-red-500", move || read_money.get() < 0)
                                                    class=("text-green-500", move ||read_money.get() >= 0)

                                                >{move || User::calc_money(read_money.get())}"€"</p>
                                                <div class="flex place-content-evenly">
                                                </div>
                                            </div>
                                            <div class="col-span-1">
                                                <a href=format!("/user/{}/settings", user_id) class="text-white pt-3">{SettingsIcon()}</a>
                                            </div>
                                        </div>
                                    </div>
                                    <div>
                                        // right side (put in money)
                                        <div class="flex flex-col gap-3 bg-gray-500 p-3 rounded-[10px]">
                                            <div class="grid grid-cols-3 gap-5 rounded-[10px]">
                                                {change_money_button(50, args.clone())}
                                                {change_money_button(100, args.clone())}
                                                {change_money_button(200, args.clone())}
                                                {change_money_button(500, args.clone())}
                                                {change_money_button(1000, args.clone())}
                                                {change_money_button(2000, args.clone())}
                                                {change_money_button(5000, args.clone())}

                                            </div>
                                            <div class="grid grid-cols-3 gap-3">
                                                <a href="#" class="bg-red-400 text-white rounded-full p-5" on:click=move |_| on_custom_money_button_click(false, custom_money_change, &args1)>
                                                    <div class="pad-5 text-center">
                                                        "-"
                                                    </div>
                                                </a>
                                                <input class="text-center rounded-[10px]" placeholder="Euro eingeben" bind:value=custom_money_change/>
                                                <a href="#" class="bg-emerald-600 text-white rounded-full p-5" on:click=move |_| on_custom_money_button_click(true, custom_money_change, &args2)>
                                                    <div class="pad-5 text-center">
                                                        "+"
                                                    </div>
                                                </a>
                                            </div>
                                            <div class="grid grid-cols-3 gap-5 rounded-[10px]">
                                                {change_money_button(-50, args.clone())}
                                                {change_money_button(-100, args.clone())}
                                                {change_money_button(-200, args.clone())}
                                                {change_money_button(-500, args.clone())}
                                                {change_money_button(-1000, args.clone())}
                                                {change_money_button(-2000, args.clone())}
                                                {change_money_button(-5000, args.clone())}

                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                </div>
                </Suspense>
            }
        }.into_any()
        }
    }
    .into_any();
}

#[component]
pub fn SettingsIcon() -> impl IntoView {
    view! {
        <svg width="50px" height="50px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M15 12C15 13.6569 13.6569 15 12 15C10.3431 15 9 13.6569 9 12C9 10.3431 10.3431 9 12 9C13.6569 9 15 10.3431 15 12Z" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="--darkreader-inline-stroke: var(--darkreader-text-000000, #e8e6e3);" data-darkreader-inline-stroke=""></path> <path d="M12.9046 3.06005C12.6988 3 12.4659 3 12 3C11.5341 3 11.3012 3 11.0954 3.06005C10.7942 3.14794 10.5281 3.32808 10.3346 3.57511C10.2024 3.74388 10.1159 3.96016 9.94291 4.39272C9.69419 5.01452 9.00393 5.33471 8.36857 5.123L7.79779 4.93281C7.3929 4.79785 7.19045 4.73036 6.99196 4.7188C6.70039 4.70181 6.4102 4.77032 6.15701 4.9159C5.98465 5.01501 5.83376 5.16591 5.53197 5.4677C5.21122 5.78845 5.05084 5.94882 4.94896 6.13189C4.79927 6.40084 4.73595 6.70934 4.76759 7.01551C4.78912 7.2239 4.87335 7.43449 5.04182 7.85566C5.30565 8.51523 5.05184 9.26878 4.44272 9.63433L4.16521 9.80087C3.74031 10.0558 3.52786 10.1833 3.37354 10.3588C3.23698 10.5141 3.13401 10.696 3.07109 10.893C3 11.1156 3 11.3658 3 11.8663C3 12.4589 3 12.7551 3.09462 13.0088C3.17823 13.2329 3.31422 13.4337 3.49124 13.5946C3.69158 13.7766 3.96395 13.8856 4.50866 14.1035C5.06534 14.3261 5.35196 14.9441 5.16236 15.5129L4.94721 16.1584C4.79819 16.6054 4.72367 16.829 4.7169 17.0486C4.70875 17.3127 4.77049 17.5742 4.89587 17.8067C5.00015 18.0002 5.16678 18.1668 5.5 18.5C5.83323 18.8332 5.99985 18.9998 6.19325 19.1041C6.4258 19.2295 6.68733 19.2913 6.9514 19.2831C7.17102 19.2763 7.39456 19.2018 7.84164 19.0528L8.36862 18.8771C9.00393 18.6654 9.6942 18.9855 9.94291 19.6073C10.1159 20.0398 10.2024 20.2561 10.3346 20.4249C10.5281 20.6719 10.7942 20.8521 11.0954 20.94C11.3012 21 11.5341 21 12 21C12.4659 21 12.6988 21 12.9046 20.94C13.2058 20.8521 13.4719 20.6719 13.6654 20.4249C13.7976 20.2561 13.8841 20.0398 14.0571 19.6073C14.3058 18.9855 14.9961 18.6654 15.6313 18.8773L16.1579 19.0529C16.605 19.2019 16.8286 19.2764 17.0482 19.2832C17.3123 19.2913 17.5738 19.2296 17.8063 19.1042C17.9997 18.9999 18.1664 18.8333 18.4996 18.5001C18.8328 18.1669 18.9994 18.0002 19.1037 17.8068C19.2291 17.5743 19.2908 17.3127 19.2827 17.0487C19.2759 16.8291 19.2014 16.6055 19.0524 16.1584L18.8374 15.5134C18.6477 14.9444 18.9344 14.3262 19.4913 14.1035C20.036 13.8856 20.3084 13.7766 20.5088 13.5946C20.6858 13.4337 20.8218 13.2329 20.9054 13.0088C21 12.7551 21 12.4589 21 11.8663C21 11.3658 21 11.1156 20.9289 10.893C20.866 10.696 20.763 10.5141 20.6265 10.3588C20.4721 10.1833 20.2597 10.0558 19.8348 9.80087L19.5569 9.63416C18.9478 9.26867 18.6939 8.51514 18.9578 7.85558C19.1262 7.43443 19.2105 7.22383 19.232 7.01543C19.2636 6.70926 19.2003 6.40077 19.0506 6.13181C18.9487 5.94875 18.7884 5.78837 18.4676 5.46762C18.1658 5.16584 18.0149 5.01494 17.8426 4.91583C17.5894 4.77024 17.2992 4.70174 17.0076 4.71872C16.8091 4.73029 16.6067 4.79777 16.2018 4.93273L15.6314 5.12287C14.9961 5.33464 14.3058 5.0145 14.0571 4.39272C13.8841 3.96016 13.7976 3.74388 13.6654 3.57511C13.4719 3.32808 13.2058 3.14794 12.9046 3.06005Z" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="--darkreader-inline-stroke: var(--darkreader-text-000000, #e8e6e3);" data-darkreader-inline-stroke=""></path> </g></svg>
    }
}

fn change_money_button(
    money: i64,
    args: Rc<MoneyArgs>
) -> impl IntoView {
    view! {
        <a
            on:click=move |_| change_money_logic(money, args.clone())
            href="#"
            class="p-5 text-white rounded-[10px] text-center text-[1.25em]"
            class=("bg-emerald-600", move || money > 0)
            class=("bg-red-400", move || money < 0)
        >{User::calc_money(money)}"€"</a>
    }
}

fn change_money_logic(money: i64, args: Rc<MoneyArgs>){
    let user_id = args.user_id.clone();
    let money_write = args.money_write;
    let money_read = args.money_read;
    let error_write = args.error_write;

    change_money_logic_raw(money, user_id, money_write, money_read, error_write);
}

fn change_money_logic_raw(money: i64, user_id: i64, money_write: WriteSignal<i64>, money_read: ReadSignal<i64>, error_write: WriteSignal<String>){
    spawn_local(async move {
        let mut t_type = if money > 0 { TransactionType::DEPOSIT } else { TransactionType::WITHDRAW };
        
        let resp = create_transaction(user_id, money, t_type).await;

        if resp.is_ok() {
            money_write.set(money_read.get_untracked() + money);
            error_write.set(String::new())
        } else {
            let error = resp.err().unwrap().to_string();

            error_write.set(error);

        }
    })
}

fn on_custom_money_button_click(add: bool, value: RwSignal<String>, args: &MoneyArgs){
    let string = value.get_untracked();

    let error_write = args.error_write;
    error_write.set(String::new());

    if string.len() == 0 {
        return;
    }

    let (mut euros, mut cents): (String, String) = (0.to_string(), 0.to_string());

    let string = string.replace(",", ".");

    let split = string.rsplit_once(".");

    if split.is_none() {
        euros = string;
    } else {
        let split = split.unwrap();
        (euros, cents) = (split.0.to_string(), split.1.to_string());
    }

    if euros.len() == 0 {
        error_write.set("Failed to parse euros".to_string());
        return;
    }

    if cents.len() == 0 {
        error_write.set("Failed to parse cents".to_string());
        return;
    }

    if cents.len() > 2 {
        cents.truncate(2);
    }

    if cents.len() < 2 {
        cents.push_str("0");
    }

    let real_euros = euros.parse::<i64>();
    if real_euros.is_err() {
        error_write.set(format!("Failed to parse euros: {}", euros));
        return;
    }

    let real_cents = cents.parse::<i64>();

    if real_cents.is_err() {
        error_write.set(format!("Failed to parse cents: {}", cents));
        return;
    }

    let real_euros = real_euros.unwrap();
    let real_cents = real_cents.unwrap();

    // console_log(&format!("Need to modify {}€ and {} cents", real_euros, real_cents));

    let mut final_cents = real_euros * 100 + real_cents;

    if !add {
        final_cents = -final_cents;
    }

    change_money_logic_raw(final_cents, args.user_id, args.money_write, args.money_read, args.error_write);

    value.set(String::new());    
}
