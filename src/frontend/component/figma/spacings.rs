pub enum Spacing {
    XXXS,
    XS,
    S,
    L,
    M,
    XXL,
}

impl Spacing {
    pub fn get(&self) -> String {
        (match self {
            Spacing::L => "40px",
            Spacing::M => "30px",
            Spacing::XXL => "80px",
            Spacing::XS => "10px",
            Spacing::S => "20px",
            Spacing::XXXS => "2.5px",
        })
        .to_string()
    }
}
