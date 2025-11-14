// colors as defined in figma
pub enum Colors {
    BackgroundLight,
}

impl Colors {
    pub fn get(self) -> String {
        (match self {
            Colors::BackgroundLight => "#2B2B2B",
        })
        .to_string()
    }
}
