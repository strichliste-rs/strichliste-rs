use leptos::prelude::*;

use crate::backend::core::behaviour::article_get_all::get_all_articles;

#[component]
pub fn ShowArticles() -> impl IntoView {
    let all_articles = OnceResource::new(get_all_articles(None));
    view! {
        <Suspense fallback=move || {
            view! { <h1>"Loading articles..."</h1> }
        }>
            {move || {
                all_articles
                    .get()
                    .map(|articles| {
                        match articles {
                            Err(err) => {
                                let msg = match err {
                                    ServerFnError::ServerError(msg) => msg,
                                    _ => err.to_string(),
                                };
                                view! {
                                    <p class="text-red-900">"Failed to fetch article: " {msg}</p>
                                }
                                    .into_any()
                            }
                            Ok(mut articles) => {
                                view! {
                                    <table class="w-full text-white p-2">
                                        <thead>
                                            <tr class="bg-black">
                                                <th>"Name"</th>
                                                <th>"Preis"</th>
                                                <th></th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {
                                                articles.sort_by(|a, b| a.name.cmp(&b.name));
                                                articles
                                                    .into_iter()
                                                    .map(|article| {
                                                        view! {
                                                            <tr class="even:bg-gray-700 odd:bg-gray-500">
                                                                <td class="p-2 text-center">{article.name}</td>
                                                                <td class="p-2 text-center">{article.cost.format_eur()}</td>
                                                                <td class="bg-green-700 p-2">
                                                                    <a href=format!("/articles/{}", article.id)>
                                                                        <p class="text-center">"Edit"</p>
                                                                    </a>
                                                                </td>
                                                            </tr>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        </tbody>
                                    </table>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}
