use crate::{
    constants::css_queries,
    models::{
        build_args::BuildArgs, champion_settings::ChampionSettings, rune_setup::RuneSetup,
        rune_tree::RuneTree,
    },
};
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

use super::url_builder;

pub async fn scrape_champion_settings(build_args: &BuildArgs) -> ChampionSettings {
    let url = url_builder::build_url(build_args);
    let body_response = get_body_response(url).await;
    let document = Document::from(&body_response[..]);
    let champion_settings = get_champion_settings(&document);

    champion_settings
}

fn parse_perk(perk: &str) -> String {
    let mut parsed_perk = perk.to_owned();
    parsed_perk = parsed_perk.replace("The Keystone ", "");
    parsed_perk = parsed_perk.replace("The Rune ", "");
    parsed_perk = parsed_perk.replace(" Shard", "");
    parsed_perk = parsed_perk.replace("The ", "");

    parsed_perk
}

async fn get_body_response(url: String) -> String {
    let response = reqwest::get(url).await.unwrap();
    let body_response = response.text().await.unwrap();

    return body_response;
}

fn get_champion_settings(document: &Document) -> ChampionSettings {
    let rune_setup = get_rune_setup(&document);
    let skill_setup = get_spells_or_skills(&document, css_queries::SKILL_PRIORITY_PATH);
    let spell_order = get_spells_or_skills(&document, css_queries::SUMMONER_SPELLS);

    ChampionSettings {
        skill_priority: skill_setup,
        rune_trees: rune_setup,
        spells: spell_order,
    }
}

fn get_rune_setup(document: &Document) -> RuneSetup {
    let primary_tree = get_rune_specifications(&document, css_queries::PRIMARY_TREE);
    let secondary_tree = get_rune_specifications(&document, css_queries::SECONDARY_TREE);

    RuneSetup {
        primary_tree,
        secondary_tree,
    }
}

fn get_rune_specifications(document: &Document, query_selector: &str) -> RuneTree {
    let mut title: String = "".to_string();
    let mut perks = Vec::new();
    for node in document.find(Class(query_selector)).take(1) {
        title = node
            .find(Class(css_queries::PERK_STYLE_TITLE))
            .next()
            .unwrap()
            .text();

        perks.append(&mut get_perk(&node, css_queries::PERK_ACTIVE));
        perks.append(&mut get_perk(&node, css_queries::SHARD_ACTIVE));
    }

    RuneTree {
        title: title,
        perks: perks,
    }
}

fn get_perk(node: &Node, query_selector: &str) -> Vec<String> {
    let mut perks: Vec<String> = Vec::new();
    for perk_node in node.find(Class(query_selector)) {
        let perk_image = perk_node.find(Name(css_queries::IMG)).next().unwrap();
        let perk_value = perk_image.attr(css_queries::ALT).unwrap();
        let parsed_perk = parse_perk(perk_value);
        perks.push(parsed_perk);
    }

    perks
}

fn get_spells_or_skills(document: &Document, query_selector: &str) -> Vec<String> {
    let mut spells: Vec<String> = Vec::new();
    for node in document.find(Class(query_selector)).take(1) {
        for img in node.find(Name(css_queries::IMG)) {
            let mut spell = img.attr(css_queries::ALT).unwrap().to_owned();
            spell = spell.replace("Summoner Spell ", "");
            spells.push(spell.to_string());
        }
    }

    spells
}
