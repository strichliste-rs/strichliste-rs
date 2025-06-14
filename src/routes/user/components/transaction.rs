use std::rc::Rc;

use leptos::prelude::*;

use crate::{models::Article, routes::user::MoneyArgs};

pub fn buy_item(user_id: i64, article: &Article, args: Rc<MoneyArgs>) {}
