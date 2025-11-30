#[non_exhaustive]
pub struct Spacing;

// browser default: 1rem = 16px
impl Spacing {
    pub const XXXS: &'static str = "0.15625rem";
    pub const XS: &'static str = "0.625rem";
    pub const S: &'static str = "1.25rem";
    pub const M: &'static str = "1.875rem";
    pub const L: &'static str = "2.5rem";
    pub const XXL: &'static str = "5rem";

    #[inline(always)] // tried to make it a `const fn`, but no luck. Even with the `const_format` crate, since it doesn't accpet precision specifiers.
    pub fn px_to_rem(px_value: i32) -> String {
        let rem_value: f32 = px_value as f32 / 16.0;
        format!("{:.5}rem", rem_value)
    }
}
