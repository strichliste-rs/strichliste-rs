use itertools::Itertools;
use leptos::prelude::*;
use leptos_meta::Style;
use thaw::{Table, TableBody, TableCell, TableRow};

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

    let users_by_letter = move |letter: &String| {
        users
            .read()
            .value
            .get()
            .into_iter()
            .filter(|user| {
                user.nickname
                    .to_lowercase()
                    .starts_with(&letter.to_lowercase())
            })
            .collect_vec()
    };

    view! {
        <Style>
            r#"
                .thaw-table-cell {
                    padding-bottom: 3%;
                    padding-top: 3%;
                }
            "#
        </Style>
        <div
            style:background-color=Color::BackgroundDark.get()
            style:padding="5% 10%"
            style:padding-top="0"
        >
            {move || {
                first_letters
                    .get()
                    .into_iter()
                    .map(|char| {
                        let users = RwSignal::new(users_by_letter(&char));
                        view! {
                            <div>
                                <h1
                                    style:padding-top="8%"
                                    style:font-size=Spacing::L.get()
                                    style:font-weight="700"
                                    style:padding-bottom="8%"
                                    style:max-height="54px"
                                >
                                    {char}
                                </h1>
                                <Table>
                                    <TableBody>
                                        {move || {
                                            users
                                                .get()
                                                .into_iter()
                                                .map(|user| {
                                                    view! {
                                                        <TableRow style:font-size=Spacing::M.get()>
                                                            <TableCell style:text-align="left">
                                                                <p>{user.nickname}</p>
                                                            </TableCell>
                                                            <TableCell style:text-align="right">
                                                                <p
                                                                    class=("text-red-500", move || user.money.value < 0)
                                                                    class=("text-green-500", move || user.money.value >= 0)
                                                                >
                                                                    {user.money.format_eur()}
                                                                </p>
                                                            </TableCell>

                                                        </TableRow>
                                                    }
                                                })
                                                .collect_view()
                                        }}
                                    </TableBody>

                                </Table>

                            </div>
                        }
                    })
                    .collect_view()
            }}

        </div>
    }
}
