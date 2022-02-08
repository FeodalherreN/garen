use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutRunePage {
    pub name: String,
    pub current: bool,
    pub primary_style_id: i64,
    pub selected_perk_ids: Vec<i64>,
    pub sub_style_id: i64,
}
