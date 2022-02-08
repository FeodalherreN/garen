use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LcuRunes {
    pub primary: String,
    pub perks: Vec<String>,
    pub secondary: String,
}
