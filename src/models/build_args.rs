use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BuildArgs {
    pub champion: String,
    pub mode: Option<String>,
    pub role: Option<String>,
}
impl BuildArgs {
    pub(crate) fn new() -> BuildArgs {
        BuildArgs {
            champion: "".to_string(),
            mode: None,
            role: None,
        }
    }
}
