use std::rc::Rc;

use leptos::{ev, html, prelude::*};

use crate::{
    backend::core::{behaviour::article_get_all::get_all_articles, Article},
    frontend::{model::money_args::MoneyArgs, shared::buy_article},
};

#[component]
pub fn ArticleSearch(money_args: Rc<MoneyArgs>) -> impl IntoView {
    let MoneyArgs {
        user_id,
        money,
        transactions,
    } = *money_args;
    let articles_resource = OnceResource::new(get_all_articles(None));

    let dropdown_div = NodeRef::<html::Div>::new();
    let search_term = RwSignal::new(String::new());
    let filtered_articles = RwSignal::new(Vec::<Article>::new());

    let articles_signal = RwSignal::new(Vec::<Article>::new());

    let on_input = move |_ev: ev::Event| {
        match search_term.get().len() {
            0 => filtered_articles.set(Vec::<Article>::new()),
            _ => filtered_articles.update(|val| {
                *val = articles_signal
                    .get()
                    .iter()
                    .filter(|elem| {
                        elem.name
                            .to_lowercase()
                            .contains(&search_term.get().to_lowercase())
                    })
                    .take(5)
                    .cloned()
                    .collect::<Vec<Article>>();
            }),
        };
    };

    view! {
        {move || {
            articles_resource
                .get()
                .map(|value| {
                    match value {
                        Ok(value) => {
                            articles_signal.set(value);
                            ().into_any()
                        }
                        Err(e) => {
                            let msg = match e {
                                ServerFnError::ServerError(msg) => msg,
                                _ => e.to_string(),
                            };

                            view! {
                                <p class="bg-red-400 text-white text-center">
                                    {format!("Failed to fetch articles: {}", msg)}
                                </p>
                            }
                                .into_any()
                        }
                    }
                })
        }}
        <div class="w-full min-w-[200px] flex flex-col items-center p-2">
            <div class="relative">
                <input
                    class="text-white w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md pl-3 pr-28 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                    placeholder="Search for articles"
                    autocomplete=false
                    bind:value=search_term
                    on:input=on_input
                />
                <button
                    class="absolute top-1 right-1 flex items-center rounded bg-slate-800 py-1 px-2.5 border border-transparent text-center text-sm text-white transition-all shadow-sm hover:shadow focus:bg-slate-700 focus:shadow-none active:bg-slate-700 hover:bg-slate-700 active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
                    type="button"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        class="w-4 h-4 mr-2"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z"
                            clip-rule="evenodd"
                        />
                    </svg>

                    Search
                </button>
            </div>
            <div node_ref=dropdown_div class=("hidden", move || search_term.get().is_empty())>
                {move || {
                    filtered_articles
                        .get()
                        .into_iter()
                        .map(|elem| {
                            view! {
                                <button on:click=move |_| {
                                    buy_article(user_id, elem.id, money, transactions);
                                    search_term.set(String::new());
                                }>
                                    <div class="p-2 m-2 rounded text-white bg-gray-700">
                                        <p>{elem.name.clone()}" | "{elem.cost.format_eur()}</p>
                                    </div>
                                </button>
                            }
                                .into_any()
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }.into_any()
}
