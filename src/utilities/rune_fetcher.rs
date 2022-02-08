use crate::models::lcu_runes::LcuRunes;
use select::{
    document::Document,
    predicate::{Class, Name},
};

pub async fn scrape_runes(champion: &str) -> LcuRunes {
    let body_response = get_body_response(champion).await;
    let lcu_runes = scrape_body_response(body_response);

    lcu_runes
}

fn scrape_body_response(body_response: String) -> LcuRunes {
    let body_resp: &str = &body_response[..];
    let document = Document::from(body_resp);
    let rune_titles = get_rune_titles(document.to_owned());
    let perks = get_perks(document.to_owned());

    let lcu_runes = LcuRunes {
        primary: rune_titles[0].to_owned(),
        perks: perks.to_owned(),
        secondary: rune_titles[1].to_owned(),
    };

    lcu_runes
}

fn get_perks(document: Document) -> Vec<String> {
    let mut perks: Vec<String> = Vec::new();
    for node in document.find(Class("perk-active")).take(6) {
        let perk_image = node.find(Name("img")).next().unwrap();
        let perk_value = perk_image.attr("alt").unwrap();
        let parsed_perk = parse_perk(perk_value);
        perks.push(parsed_perk);
    }

    for node in document.find(Class("shard-active")).take(3) {
        let shard_image = node.find(Name("img")).next().unwrap();
        let shard_value = shard_image.attr("alt").unwrap();
        let parsed_shard = parse_perk(shard_value);
        perks.push(parsed_shard);
    }

    return perks;
}

fn parse_perk(perk: &str) -> String {
    let mut parsed_perk = perk.to_owned();
    parsed_perk = parsed_perk.replace("The Keystone ", "");
    parsed_perk = parsed_perk.replace("The Rune ", "");
    parsed_perk = parsed_perk.replace(" Shard", "");
    parsed_perk = parsed_perk.replace("The ", "");

    parsed_perk
}

fn get_rune_titles(document: Document) -> Vec<String> {
    let mut titles: Vec<String> = Vec::new();
    for node in document.find(Class("perk-style-title")).take(2) {
        titles.push(node.text());
    }

    titles
}

async fn get_body_response(champion: &str) -> String {
    let url = format!("https://u.gg/lol/champions/aram/{}-aram", champion);
    let response = reqwest::get(url).await.unwrap();
    let body_response = response.text().await.unwrap();

    return body_response;
}
