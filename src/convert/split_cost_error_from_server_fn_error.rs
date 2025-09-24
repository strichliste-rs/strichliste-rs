use crate::model::SplitCostError;
use leptos::prelude::*;

impl FromServerFnError for SplitCostError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}
