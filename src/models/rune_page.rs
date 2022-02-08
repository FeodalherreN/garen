use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunePage {
    pub auto_modified_selections: Vec<i64>,
    pub current: bool,
    pub id: i64,
    pub is_active: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_valid: bool,
    pub last_modified: i64,
    pub name: String,
    pub order: i64,
    pub primary_style_id: i64,
    pub selected_perk_ids: Vec<i64>,
    pub sub_style_id: i64,
}
