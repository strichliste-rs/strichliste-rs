use leptos::prelude::*;

#[component]
pub fn View() -> impl IntoView {
    view! {
        <div class="grid grid-cols-10 gap-10 py-10">
            <div class="col-span-1 pl-5">
                <a href="/articles/create">
                    <div class="flex justify-center">
                        // joinked from: https://gist.github.com/ibelick/0c92c1aba54c2f7e8b3a7381426ed029
                        <button class="inline-flex h-10 w-10 items-center justify-center rounded-full bg-gray-50 text-black drop-shadow-sm transition-colors duration-150 hover:bg-gray-200">
                            "+"
                        </button>
                    </div>
                </a>
            </div>
            <div class="col-span-9 pr-7">
                <ShowArticles/>
            </div>
        </div>
    }
}

#[component]
fn ShowArticles() -> impl IntoView {
    view! {}
}
