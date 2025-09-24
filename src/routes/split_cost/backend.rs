use leptos::prelude::*;

use crate::{model::SplitCostError, routes::user::CreateTransactionError};

#[cfg(feature = "ssr")]
use crate::backend::database::DBError;

impl FromServerFnError for SplitCostError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}

impl From<CreateTransactionError> for SplitCostError {
    fn from(value: CreateTransactionError) -> Self {
        Self::CreateTransactionError(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<DBError> for SplitCostError {
    fn from(value: DBError) -> Self {
        Self::DatabaseError(value.to_string())
    }
}
