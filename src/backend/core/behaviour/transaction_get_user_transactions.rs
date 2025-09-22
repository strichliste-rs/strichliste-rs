#![cfg(feature = "ssr")]

use crate::{
    backend::database::{ArticleDB, DatabaseResponse, TransactionDB, DB},
    model::{Page, PageRequestParams, UserId},
    models::{Transaction, TransactionType},
};

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

                    // this shows the user his transferred amount when a group transaction was made
                    transaction.money.value /= sender_group.members.len() as i64;
                }

                _ => {}
            }
        }

        Ok(Page::new(page_request_params, total, transactions))
    }
}
