//replace this with tracing levels as soon as arbitrary const types get stable

pub const THROW_ERROR_SOFT: u8 = 0;
pub const THROW_ERROR_HARD: u8 = 1;

#[derive(Default, Clone)]
pub struct ThrowError<const L: u8>(pub Vec<String>);
