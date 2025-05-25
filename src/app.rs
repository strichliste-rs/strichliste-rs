use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

use crate::routes::{self, state::FrontendStore, user};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-[#25333f]">
                <App/>
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
        <Stylesheet id="leptos" href="/pkg/strichliste-rs.css"/>

        // sets the document title
        <Title text="Strichliste-rs"/>

        {routes::navbar::View()}

        // content for this welcome page
        <Router>
            <Routes fallback=|| view!{ <h1>"Page not found!"</h1>}>
                <Route path=path!("/") view=routes::home::View/>
                <Route path=path!("/create_user") view=routes::create_user::View/>
                <Route path=path!("/user/:id") view=routes::user::ShowUser/>
                <Route path=path!("/user/:id/settings") view=routes::user::settings::Show/>
            </Routes>
        </Router>
    }
}
