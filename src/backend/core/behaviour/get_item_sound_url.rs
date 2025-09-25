use leptos::prelude::*;

use crate::model::AudioPlayback;

#[server]
pub async fn get_item_sound_url(audio: AudioPlayback) -> Result<Vec<u8>, ServerFnError> {
    use crate::backend::core::{
        behaviour::article_get::get_article, misc::choose_random_item, ServerState,
    };
    use axum::http::StatusCode;
    use leptos_axum::ResponseOptions;
    use std::path::PathBuf;
    use std::str::FromStr;
    use tracing::error;

    let response_opts: ResponseOptions = expect_context();

    let state: ServerState = expect_context();

    let sounds = &state.settings.sounds;

    let file = match audio {
        AudioPlayback::Failed => choose_random_item(&sounds.failed),
        AudioPlayback::Undo => choose_random_item(&sounds.generic),
        AudioPlayback::Deposit(_) => choose_random_item(&sounds.generic),
        AudioPlayback::Sent(_) => choose_random_item(&sounds.generic),
        AudioPlayback::Withdraw(_) => choose_random_item(&sounds.generic),
        AudioPlayback::Bought(article_id) => {
            let article = get_article(article_id).await?;

            let sounds = match sounds.articles.get(&article.name) {
                Some(sounds) => sounds,
                None => &sounds.generic,
            };

            choose_random_item(sounds)
        }
    };

    let path = PathBuf::from_str(match file {
        Some(val) => val,
        None => {
            error!("Failed to choose a random sound file");
            response_opts.set_status(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(ServerFnError::new("Failed to get sound file"));
        }
    });

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
