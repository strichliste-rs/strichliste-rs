use std::collections::HashMap;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use thaw::{ssr::SSRMountStyleProvider, ConfigProvider, Theme, ToasterProvider};

use crate::frontend::{
    component::{
        error_popup::ErrorDisplay, error_soft::ErrorSoftDisplay, figma::colors::Color,
        i18n_provider::I18nProvider,
    },
    model::{
        caching_layer::CachingLayer,
        frontend_store::FrontendStore,
        scaninput_manager::ScanInputManager,
        throw_error::{ThrowError, THROW_ERROR_HARD, THROW_ERROR_SOFT},
    },
    route::{self},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                // otherwise the sceen will flash white when loading a user for example
                <body style:background-color=Color::BACKGROUND_DARK>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    use reactive_stores::Store;
    let audio_ref = NodeRef::<leptos::html::Audio>::new();
    let store = Store::new(FrontendStore {
        cached_sounds: Default::default(),
        audio_ref,
        cachinglayer: RwSignal::new(CachingLayer::default()),
    });
    let soft_error = Store::new(ThrowError::<THROW_ERROR_SOFT>::default());
    let hard_error = Store::new(ThrowError::<THROW_ERROR_HARD>::default());
    provide_context(store);
    provide_context(soft_error);
    provide_context(hard_error);

    let colors = RwSignal::new(HashMap::from([
        (10, "#010304"),
        (20, "#0F181E"),
        (30, "#192731"),
        (40, "#25333F"),
        (50, "#323F4C"),
        (60, "#3F4B59"),
        (70, "#4D5866"),
        (80, "#5B6573"),
        (90, "#6A7280"),
        (100, "#79808D"),
        (110, "#888D9B"),
        (120, "#979CA8"),
        (130, "#A7AAB5"),
        (140, "#B7B9C3"),
        (150, "#C7C8D0"),
        (160, "#D7D7DD"),
    ]));
    let mut theme = Theme::custom_dark(&colors.get_untracked());
    theme
        .color
        .set_color_neutral_background_1(Color::BACKGROUND_DARK.to_string());
    let theme = RwSignal::new(theme);

    let scaninput_manager = Store::new(ScanInputManager::default());

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/strichliste-rs.css" />

        <Title text="Strichliste-rs" />

        <audio node_ref=audio_ref />

        <ConfigProvider theme>
            <I18nProvider>
                <ToasterProvider>
                    <ErrorDisplay />
                    <ErrorSoftDisplay />
                    <Router>
                        {
                            ScanInputManager::setup(scaninput_manager);
                            provide_context(scaninput_manager);
                        }
                        <Routes fallback=|| {
                            view! {
                                <h1 class="text-white text-center bg-red-400">"Page not found!"</h1>
                            }
                        }>
                            <Route path=path!("/") view=route::home::View />
                            <Route path=path!("/user/:id") view=route::user::ShowUser />
                            <Route
                                path=path!("/user/:id/settings")
                                view=route::user::settings::Show
                            />
                            <Route
                                path=path!("/user/:id/send_money")
                                view=route::user::send_money::Show
                            />
                            <Route path=path!("/articles") view=route::articles::View />
                            <Route
                                path=path!("/articles/create")
                                view=route::articles::create::Create
                            />
                            <Route path=path!("/articles/:article_id") view=route::articles::Edit />

                            <Route path=path!("/split_cost") view=route::split_cost::Show />
                        </Routes>
                    </Router>
                </ToasterProvider>
            </I18nProvider>
        </ConfigProvider>
    }
}
