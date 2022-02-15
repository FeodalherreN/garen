use serde::Deserialize;

use super::rune_setup::RuneSetup;

#[derive(Deserialize, Debug)]
pub struct ChampionSettings {
    pub skill_priority: Vec<String>,
    pub rune_trees: RuneSetup,
    pub spells: Vec<String>,
}
