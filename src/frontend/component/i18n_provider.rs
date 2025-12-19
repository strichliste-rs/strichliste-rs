use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "de",
        check_translations: "./src/**/*.rs",
    }
}
