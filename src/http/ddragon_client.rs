use crate::models::runes_reforged::RunesReforged;

pub async fn get_runes_reforged() -> Vec<RunesReforged> {
    let latest_version = get_latest_version().await;
    let url = format!(
        "https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/runesReforged.json",
        latest_version
    );
    let response = reqwest::get(url).await.unwrap();
    let runes_reforged = response.json::<Vec<RunesReforged>>().await.unwrap();

    runes_reforged
}

pub async fn get_latest_version() -> String {
    let url = "https://ddragon.leagueoflegends.com/api/versions.json";
    let response = reqwest::get(url).await.unwrap();
    let runes_reforged = response.json::<Vec<String>>().await.unwrap();
    let latest_version = runes_reforged[0].to_owned();

    latest_version
}
