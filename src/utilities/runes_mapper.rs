use crate::models::runes_reforged::RunesReforged;

pub fn map_runes(rune: String, runes_reforged: Vec<RunesReforged>) -> i64 {
    for rn in runes_reforged.into_iter() {
        if rn.name == rune {
            return rn.id;
        }
    }

    0
}

pub fn map_slots(slots: Vec<String>, runes_reforged: Vec<RunesReforged>) -> Vec<i64> {
    slots
        .into_iter()
        .map(|rune| get_slot_id_by_name(rune, &runes_reforged))
        .collect()
}

fn get_slot_id_by_name(slot: String, runes_reforged: &Vec<RunesReforged>) -> i64 {
    let hardcoded_shard_value = get_hardcoded_shard_value(slot.to_owned());
    if hardcoded_shard_value != 0 {
        return hardcoded_shard_value;
    }

    for rn in runes_reforged.into_iter() {
        for sl in rn.slots.to_owned().into_iter() {
            for rnn in sl.runes {
                if rnn.name == slot {
                    return rnn.id;
                }
            }
        }
    }

    0
}

fn get_hardcoded_shard_value(name: String) -> i64 {
    if name == "Attack Speed" {
        return 5005;
    }
    if name == "Adaptive Force" {
        return 5008;
    }
    if name == "Scaling CDR" {
        return 5007;
    }
    if name == "Armor" {
        return 5002;
    }
    if name == "Magic Resist" {
        return 5003;
    }
    if name == "Scaling Bonus Health" {
        return 5001;
    }

    0
}
