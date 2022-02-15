use serde::Deserialize;

use super::rune_tree::RuneTree;

#[derive(Deserialize, Debug)]
pub struct RuneSetup {
    pub primary_tree: RuneTree,
    pub secondary_tree: RuneTree,
}
