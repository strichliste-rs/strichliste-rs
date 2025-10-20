use leptos::prelude::FromServerFnError;

use crate::model::GetUserPreferencesError;

impl FromServerFnError for GetUserPreferencesError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: leptos::prelude::ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}
