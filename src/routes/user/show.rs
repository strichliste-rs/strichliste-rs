use std::rc::Rc;

use leptos::{prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use tracing::error;


use crate::
    models::User
;

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
pub async fn modify_money(user_id: i64, money_diff: i64) -> Result<(), ServerFnError> {
    use crate::backend::ServerState;
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;

    let state: ServerState = expect_context();

    let response_opts: ResponseOptions = expect_context();

    let user = get_user(user_id).await?;

    if user.is_none() {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new(&format!(
            "No use found with id {}",
            user_id
        )));
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

                            let args = Rc::new(MoneyArgs {
                                user_id: user_id,
                                money_read: read_money,
                                money_write: write_money,
                                error_write: error_write,
                            });

                            view!{
                                <div class="grid grid-cols-2">
                                    <div class="pt-5">
                                        // left side (show user statistics)
                                        <p class="text-center text-white text-lg">{user.nickname.clone()}</p>
                                        <p class="text-center text-lg"
                                            class=("text-red-500", move || read_money.get() < 0)
                                            class=("text-green-500", move ||read_money.get() >= 0)

                                        >{move || User::calc_money(read_money.get())}"€"</p>
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
                                                <a href="#" class="bg-red-400 text-white rounded-full p-5">
                                                    <div class="pad-5 text-center">
                                                        "-"
                                                    </div>
                                                </a>
                                                <input class="text-center rounded-[10px]" placeholder="Euro eingeben"/>
                                                <a href="#" class="bg-emerald-400 text-white rounded-full p-5">
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

fn change_money_button(
    money: i64,
    args: Rc<MoneyArgs>
) -> impl IntoView {
    view! {
        <a
            on:click=move |_| change_money_logic(money, args.clone())
            href="#"
            class="p-5 text-white rounded-[10px] text-center"
            class=("bg-emerald-400", move || money > 0)
            class=("bg-red-400", move || money < 0)
        >{User::calc_money(money)}"€"</a>
    }
}

fn change_money_logic(money: i64, args: Rc<MoneyArgs>){
    let user_id = args.user_id.clone();
    let money_write = args.money_write;
    let money_read = args.money_read;
    let error_write = args.error_write;
    spawn_local(async move {
        let resp = modify_money(user_id, money).await;

        if resp.is_ok() {
            money_write.set(money_read.get_untracked() + money);
            error_write.set(String::new())
        } else {
            let error = resp.err().unwrap().to_string();

            error_write.set(error);

        }
    })
}
