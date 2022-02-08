use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PutRunePageResponse {}
