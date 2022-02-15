use crate::models::champion_settings::ChampionSettings;

pub fn print_result(champion_settings: &ChampionSettings) {
    let table = table!(
        [bFg->champion_settings.rune_trees.primary_tree.title, bFg->champion_settings.rune_trees.secondary_tree.title, bFg->"Spells", bFg->"Skill priority"],
        [
            champion_settings.rune_trees.primary_tree.perks.join("\n"),
            champion_settings.rune_trees.secondary_tree.perks.join("\n"),
            champion_settings.spells.join("\n"),
            champion_settings.skill_priority.join("\n")
        ]
    );

    table.printstd();
}
