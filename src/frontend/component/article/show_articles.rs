use leptos::prelude::*;
use thaw::{Spinner, Table, TableBody, TableCell, TableHeader, TableRow};

use crate::{
    backend::core::behaviour::article_get_all::get_all_articles,
    frontend::shared::throw_error_none_view,
};

#[component]
pub fn ShowArticles() -> impl IntoView {
    let all_articles = OnceResource::new(get_all_articles(None));
    view! {
        <Suspense fallback=move || {
            view! { <Spinner label="Loading Articles" /> }
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
                                throw_error_none_view(format!("Failed to fetch article: {msg}"))
                            }
                            Ok(mut articles) => {
                                view! {
                                    // <table class="w-full text-white p-2">
                                    <Table>
                                        <TableHeader>
                                            <tr class="bg-black">
                                                <th>"Name"</th>
                                                <th>"Preis"</th>
                                                <th></th>
                                            </tr>
                                        </TableHeader>
                                        <TableBody>
                                            {
                                                articles
                                                    .sort_by(|a, b| {
                                                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                                    });
                                                articles
                                                    .into_iter()
                                                    .filter(|article| !article.is_disabled)
                                                    .map(|article| {
                                                        view! {
                                                            <TableRow class="even:bg-gray-700 odd:bg-gray-500">
                                                                <TableCell class="text-center">{article.name}</TableCell>
                                                                <TableCell class="text-center">
                                                                    {article.cost.format_eur()}
                                                                </TableCell>
                                                                <TableCell class="bg-green-700 p-2">
                                                                    <a href=format!("/articles/{}", article.id)>
                                                                        <p class="text-center">"Edit"</p>
                                                                    </a>
                                                                </TableCell>
                                                            </TableRow>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        </TableBody>
                                    </Table>
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}
