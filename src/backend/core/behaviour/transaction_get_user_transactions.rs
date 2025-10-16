use {
    crate::model::{Page, PageRequestParams, Transaction, UserId},
    leptos::prelude::*,
};

#[cfg(not(debug_assertions))]
use crate::backend::core::misc::custom_binary_encoding::Binary;

#[cfg(feature = "ssr")]
use crate::{
    backend::database::{ArticleDB, DatabaseResponse, TransactionDB, DB},
    model::TransactionType,
};

#[cfg(feature = "ssr")]
impl Transaction {
    pub async fn get_user_transactions(
        db: &DB,
        user_id: UserId,
        page_request_params: PageRequestParams,
    ) -> DatabaseResponse<Page<Self>> {
        use itertools::Itertools;
        use std::collections::HashMap;

        use crate::{backend::database::GroupDB, model::PageResponseParams};
        let mut conn = db.get_conn().await?;

        let user_groups = GroupDB::get_groups(&mut *conn, user_id).await?;

        let Page {
            items,
            params: PageResponseParams { total, .. },
        } = TransactionDB::get_user_transactions(&mut *conn, user_id, page_request_params).await?;
        let mut transactions = items
            .into_iter()
            .map(|elem| (elem, user_groups.as_ref()).try_into())
            .process_results(|e| e.collect::<Vec<Transaction>>())?;

        let mut article_cache = HashMap::<i64, (i64, String)>::new();

        for transaction in transactions.iter_mut() {
            match transaction.t_type {
                TransactionType::Bought(article_id) => {
                    let (price, article_name) = match article_cache.get(&article_id) {
                        None => {
                            let article =
                                match ArticleDB::get_single(&mut *conn, article_id).await? {
                                    None => continue, // Article got nuked?,
                                    Some(value) => value,
                                };

                            let price = ArticleDB::get_effective_cost(
                                &mut *conn,
                                article_id,
                                transaction.timestamp,
                            )
                            .await?;

                            let result = (price, article.name);

                            _ = article_cache.insert(article_id, result.clone());

                            result
                        }

                        Some(value) => value.clone(),
                    };

                    transaction.money = price.into();
                    transaction.description = Some(article_name);
                }

                TransactionType::Sent(_) => {
                    use crate::backend::core::Group;

                    let sender_group = Group::get(&mut *conn, transaction.group_id).await?;
                    transaction.money.value /= sender_group.members.len() as i64;
                }

                TransactionType::SentAndReceived(receiver_group) => {
                    use crate::backend::core::{Group, User};

                    let sender_group = Group::get(&mut *conn, transaction.group_id).await?;
                    let receiver_group = Group::get(&mut *conn, receiver_group).await?;

                    let transaction_db = TransactionDB::get(&mut *conn, transaction.id)
                        .await?
                        .unwrap();

                    let delta = Transaction::get_transaction_delta(
                        &mut *conn,
                        &sender_group,
                        &receiver_group,
                        &transaction_db,
                    )
                    .await?;

                    let user = User::get(&mut *conn, user_id).await?.unwrap();

                    let user_delta = delta.get(&user).unwrap();

                    dbg!(&user_delta);

                    transaction.money.value = user_delta.delta;
                }

                _ => {}
            }
        }

        Ok(Page::new(page_request_params, total, transactions))
    }
}

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn get_user_transactions(
    user_id: UserId,
    page_request_params: PageRequestParams,
) -> Result<Page<Transaction>, ServerFnError> {
    use crate::backend::core::ServerState;
    let state: ServerState = expect_context();
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::error;
    let response_opts: ResponseOptions = expect_context();

    let transactions = match Transaction::get_user_transactions(
        &*state.db.lock().await,
        user_id,
        page_request_params,
    )
    .await
    {
        Ok(transactions) => transactions,
        Err(err) => {
            error!("Failed to fetch transactions: {}", err.to_string());
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to fetch transactions!"));
        }
    };

    Ok(transactions)
}
