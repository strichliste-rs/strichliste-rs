use std::rc::Rc;

use leptos::{
    leptos_dom::logging::console_log,
    prelude::{GetUntracked, Set},
    task::spawn_local,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::Money,
    routes::user::{get_item_sound_url, MoneyArgs},
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum AudioPlayback {
    Failed,
    Undo,
    Deposit(Money),
    Sent(Money),
    Withdraw(Money),
    Bought(i64),
}

pub fn play_sound(args: Rc<MoneyArgs>, audio_playback: AudioPlayback) {
    spawn_local(async move {
        let audio = match args.audio_ref.get_untracked() {
            Some(val) => val,
            None => {
                console_log("Failed to get audio node");
                return;
            }
        };

        let sound = match get_item_sound_url(audio_playback).await {
            Ok(value) => value,
            Err(e) => {
                args.error.set(format!("Failed to fetch sound: {e}"));
                return;
            }
        };

        audio.set_src(&sound);
        match audio.play() {
            Ok(_) => {}
            Err(e) => console_log(&format!("Failed to play audio: {e:#?}")),
        }
    });
}
