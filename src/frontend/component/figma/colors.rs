use std::fmt::Display;

// colors as defined in figma
#[derive(Debug, Copy, Clone)]
pub enum Color {
    BackgroundLight,
    BackgroundDark,
    TextMain,
    Primary,
}

impl Color {
    pub fn get(&self) -> String {
        (match self {
            Color::BackgroundLight => "#2B2B2B",
            Color::TextMain => "#F0F0F0",
            Color::Primary => "#2948AE",
            Color::BackgroundDark => "#222222",
        })
        .to_string()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get())
    }
}
