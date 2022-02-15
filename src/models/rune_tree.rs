use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RuneTree {
    pub title: String,
    pub perks: Vec<String>,
}
