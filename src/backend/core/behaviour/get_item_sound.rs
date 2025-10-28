use leptos::prelude::*;

use crate::model::AudioPlayback;

#[cfg(not(debug_assertions))]
use crate::backend::core::misc::custom_binary_encoding::Binary;

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn get_item_sound_name(audio: AudioPlayback) -> Result<String, ServerFnError> {
    use crate::backend::core::{
        behaviour::article_get::get_article, misc::choose_random_item, ServerState,
    };
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use tracing::{debug, error};

    let response_opts: ResponseOptions = expect_context();

    let state: ServerState = expect_context();

    let sounds = &state.settings.sounds;

    let file = match audio {
        AudioPlayback::Failed => choose_random_item(&sounds.failed),
        AudioPlayback::Undo => choose_random_item(&sounds.generic),
        AudioPlayback::Deposit(_) => choose_random_item(&sounds.deposit),
        AudioPlayback::Sent(_) => choose_random_item(&sounds.generic),
        AudioPlayback::Withdraw(_) => choose_random_item(&sounds.withdraw),
        AudioPlayback::Bought(article_id) => {
            let article = get_article(article_id).await?;

            let sounds = match sounds.articles.get(&article.name) {
                Some(sounds) => sounds,
                None => &sounds.generic,
            };

            debug!("Picking random sound from {sounds:#?}");

            let sound = choose_random_item(sounds);

            debug!("Picked sound {sound:#?}");

            sound
        }
    };

    Ok(match file {
        None => {
            error!("Failed to choose a random sound file");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get sound file"));
        }
        Some(val) => val.to_string(),
    })
}

#[cfg_attr(not(debug_assertions), server(input=Binary, output=Binary))]
#[cfg_attr(debug_assertions, server)]
pub async fn get_item_sound_data(sound: String) -> Result<Vec<u8>, ServerFnError> {
    use crate::backend::core::ServerState;
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use std::path::PathBuf;
    use std::str::FromStr;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let state: ServerState = expect_context();

    let sounds = &state.settings.sounds;

    let all_sounds = sounds.get_all_sounds();

    // somewhat of an expensive check, but otherwise we could allow arbitrary file reads
    if !all_sounds.contains(&sound) {
        response_opts.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new(
            "Tried to fetch a sound that doesn't exist!",
        ));
    }

    let path = PathBuf::from_str(&sound);
    let path = match path {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to create PathBuf: {e}");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get sound file"));
        }
    };

    if !path.exists() {
        error!("Path '{}' does not exists!", path.to_str().unwrap());
        response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to get sound file"));
    }

    let file = match tokio::fs::read(&path).await {
        Ok(val) => val,
        Err(e) => {
            error!("Failed to read file path '{}': {e}", path.to_str().unwrap());
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get sound file"));
        }
    };

    Ok(file)
}
