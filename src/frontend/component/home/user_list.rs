use itertools::Itertools;
use leptos::prelude::*;

use crate::{
    backend::core::User,
    frontend::{
        component::figma::{colors::Color, spacings::Spacing},
        model::caching_entry::CachingEntry,
    },
};

#[component]
pub fn UserList(users: ReadSignal<CachingEntry<Vec<User>>>) -> impl IntoView {
    let first_letters = Memo::new(move |_| {
        users
            .read()
            .value
            .get()
            .into_iter()
            .map(|user| {
                user.nickname
                    .chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .to_string()
            })
            .unique()
            .collect_vec()
    });
    view! {
        <div style:background-color=Color::BackgroundDark
            .get()>
            {move || {
                first_letters
                    .get()
                    .into_iter()
                    .map(|char| {
                        view! {
                            <div>
                                <h1 style:font-size=Spacing::L.get() style:font-weight="700">
                                    {char}
                                </h1>

                            </div>
                        }
                    })
                    .collect_view()
            }}

        </div>
    }
}
