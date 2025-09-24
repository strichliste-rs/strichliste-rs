use leptos::{
    prelude::{FromServerFnError, ServerFnError, ServerFnErrorErr},
    server_fn,
};

use crate::model::CreateTransactionError;

impl FromServerFnError for CreateTransactionError {
    type Encoder = server_fn::codec::JsonEncoding;
    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}
impl From<ServerFnError> for CreateTransactionError {
    fn from(value: ServerFnError) -> Self {
        Self::StringMessage(value.to_string())
    }
}
