#![cfg(feature = "ssr")]
use rand::seq::IndexedRandom;

pub fn choose_random_item(vec: &[String]) -> Option<&String> {
    vec.choose(&mut rand::rng())
}
