use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Option<u8>,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {
        Self { r, g, b, a }
    }
}

pub fn input_opt<T: FromStr>(prompt: &str, default: &str) -> Result<Option<T>, T::Err> {
    let s: String = dialoguer::Input::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .default(default.into())
        .interact_text()
        .expect("Failed to read input");
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse().map(Some)
    }
}
