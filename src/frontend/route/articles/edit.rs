use leptos::prelude::*;

use crate::{
    backend::core::behaviour::article_get::get_article,
    frontend::{component::article::SingleArticleView, shared::throw_error_none_view},
};

#[component]
pub fn Edit() -> impl IntoView {
    use leptos_router::hooks::use_params_map;
    let params = use_params_map();
    let article_id_string = params
        .read_untracked()
        .get("article_id")
        .unwrap_or_default();

    let article_id = article_id_string.parse::<i64>();
    let article_id = match article_id {
        Ok(value) => value,
        Err(err) => {
            return throw_error_none_view(format!("Failed to get id from params map: {err}"));
        }
    };

    let article_resource = OnceResource::new(get_article(article_id));

    view! {
        {move || match article_resource.get() {
            None => view! { <p class="text-white text-center">"Loading article..."</p> }.into_any(),
            Some(value) => {
                let article = match value {
                    Ok(value) => value,
                    Err(e) => {
                        let error_msg = match e {
                            ServerFnError::ServerError(msg) => msg,
                            _ => e.to_string(),
                        };
                        return throw_error_none_view(
                            format!("Failed to load Article: {error_msg}"),
                        );
                    }
                };

                view! { <SingleArticleView article /> }
                    .into_any()
            }
        }}
    }
    .into_any()
}
