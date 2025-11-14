// colors as defined in figma
pub enum Color {
    BackgroundLight,
    TextMain,
    Primary,
}

impl Color {
    pub fn get(self) -> String {
        (match self {
            Color::BackgroundLight => "#2B2B2B",
            Color::TextMain => "#F0F0F0",
            Color::Primary => "#2948AE",
        })
        .to_string()
    }
}
