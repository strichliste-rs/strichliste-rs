use leptos::{
    leptos_dom::logging::console_log,
    prelude::{expect_context, GetUntracked},
    task::spawn_local,
};
use reactive_stores::{Store, StoreField};

use crate::{
    backend::core::behaviour::get_item_sound::{get_item_sound_data, get_item_sound_name},
    frontend::{
        model::frontend_store::{FrontendStore, FrontendStoreStoreFields},
        shared::throw_error,
    },
    model::AudioPlayback,
};

/*
   How to use a callback

   let error_callback = Closure::once(Box::new(move |_: web_sys::Event| {
       // Error playing audio, fall back to TTS
       tts_play(&tts_text);
    }));

    // Attempt to add the error event listener
    let _ = audio.add_event_listener_with_callback("error", error_callback.as_ref().unchecked_ref());
*/

pub fn play_sound(audio_playback: AudioPlayback) {
    let store = expect_context::<Store<FrontendStore>>();
    let audio = match store.audio_ref().try_get_untracked() {
        Some(v) => match v.get_untracked() {
            Some(v) => v,
            None => {
                throw_error("Failed to get audio node");
                return;
            }
        },
        None => {
            throw_error("Failed to get audio node");
            return;
        }
    };

    use leptos::web_sys::{js_sys, Url};
    spawn_local(async move {
        let mut cached_sounds = store.cached_sounds().writer().unwrap();

        let audio_file = match get_item_sound_name(audio_playback).await {
            Ok(value) => value,
            Err(e) => {
                throw_error(format!("Failed to fetch sound: {e}"));
                return;
            }
        };

        let url = match cached_sounds.get(&audio_file) {
            Some(value) => value,
            None => {
                let sound = match get_item_sound_data(audio_file.clone()).await {
                    Ok(value) => value,
                    Err(e) => {
                        throw_error(format!("Failed to fetch sound: {e}"));
                        return;
                    }
                };

                let sound: &[u8] = &sound;

                let blob = leptos::web_sys::Blob::new_with_u8_array_sequence(&js_sys::Array::of1(
                    &js_sys::Uint8Array::from(sound),
                ))
                .unwrap();

                let url = Url::create_object_url_with_blob(&blob).unwrap();

                _ = cached_sounds.insert(audio_file.clone(), url);

                cached_sounds.get(&audio_file).unwrap()
            }
        };

        audio.set_src(url);
        match audio.play() {
            Ok(_) => {}
            Err(e) => console_log(&format!("Failed to play audio: {e:#?}")),
        }
    });
}
