use itertools::Itertools;
use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::hooks::use_navigate;
use thaw::{Table, TableBody, TableCell, TableRow};

use crate::{
    backend::core::User,
    frontend::component::figma::{colors::Color, spacings::Spacing},
};

#[component]
pub fn UserList(users: Signal<Vec<User>>) -> impl IntoView {
    let first_letters = Memo::new(move |_| {
        users
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
            {format!(
                r#"
                .thaw-table-cell {{
                    padding-bottom: {0};
                    padding-top: {0};
                }}
            "#,
                Spacing::L,
            )}
        </Style>
        <div
            style:background-color=Color::BACKGROUND_DARK
            style:padding=format!("5rem {}", Spacing::XXL)
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
                                    style:padding-top=Spacing::XXL
                                    style:font-size=Spacing::L
                                    style:font-weight="700"
                                    style:padding-bottom=Spacing::L
                                    style:max-height="54px"
                                >
                                    {char}
                                </h1>
                                <Table>
                                    <TableBody>
                                        <Style>
                                            r"
                                            tr {
                                                border-bottom: initial !important;
                                            }
                                            tr:not(:last-child) {
                                                border-bottom: var(--strokeWidthThin) solid var(--colorNeutralStroke2) !important;
                                            }
                                            "#
                                        </Style>
                                        {move || {
                                            users
                                                .get()
                                                .into_iter()
                                                .map(|user| {
                                                    let navigate = use_navigate();
                                                    view! {
                                                        <TableRow
                                                            style:font-size=Spacing::M
                                                            on:click=move |_| {
                                                                navigate(&format!("/user/{}", user.id), Default::default())
                                                            }
                                                        >
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
