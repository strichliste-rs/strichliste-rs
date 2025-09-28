use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::frontend::{
    component,
    model::frontend_store::FrontendStore,
    route::{self},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-[#25333f]">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    use reactive_stores::Store;
    provide_context(Store::new(FrontendStore::default()));

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/strichliste-rs.css" />

        // sets the document title
        <Title text="Strichliste-rs" />

        {component::navbar::View()}

        // content for this welcome page
        <Router>
            <Routes fallback=|| {
                view! { <h1 class="text-white text-center bg-red-400">"Page not found!"</h1> }
            }>
                <Route path=path!("/") view=route::home::View />
                <Route path=path!("/user/create") view=route::user::Create />
                <Route path=path!("/user/:id") view=route::user::ShowUser />
                <Route path=path!("/user/:id/settings") view=route::user::settings::Show />
                <Route path=path!("/user/:id/send_money") view=route::user::send_money::Show />
                <Route path=path!("/articles") view=route::articles::View />
                <Route path=path!("/articles/create") view=route::articles::create::Create />
                <Route path=path!("/articles/:article_id") view=route::articles::Edit />

                <Route path=path!("/split_cost") view=route::split_cost::Show />
            </Routes>
        </Router>
    }
}
