use leptos::prelude::FromServerFnError;

use crate::model::SetUserPreferencesError;

impl FromServerFnError for SetUserPreferencesError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: leptos::prelude::ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}
