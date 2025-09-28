use leptos::prelude::*;

use crate::{
    backend::core::behaviour::article_new::CreateArticle, frontend::shared::throw_error_none_view,
};

#[component]
pub fn Create() -> impl IntoView {
    let create_article_action = ServerAction::<CreateArticle>::new();
    view! {
        <div class="flex h-screen bg-gray-900">
            <div class="w-full max-w-xs m-auto bg-indigo-100 rounded p-5">
                <ActionForm action=create_article_action>
                    <div>
                        <label class="block mb-2 text-indigo-500">"Name: "</label>
                        <input
                            class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300"
                            type="text"
                            name="name"
                        />
                        <label class="block mb-2 text-indigo-500">"Cost: "</label>
                        <input
                            class="w-full p-2 mb-6 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300"
                            type="text"
                            name="cost"
                        />
                    </div>
                    <div>
                        <input
                            class="w-full bg-indigo-700 hover:bg-pink-700 text-white font-bold py-2 px-4 mb-6 rounded"
                            type="submit"
                            value="Create Article"
                        />
                    </div>
                </ActionForm>
                <div>
                    {move || match create_article_action.value().get() {
                        Some(Err(e)) => {
                            let msg = match e {
                                ServerFnError::ServerError(msg) => msg,
                                _ => e.to_string(),
                            };
                            throw_error_none_view(format!("Failed to create article: {msg}"))
                        }
                        _ => ().into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}
