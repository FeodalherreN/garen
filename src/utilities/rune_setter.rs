use crate::{
    constants,
    http::{self, league_client_api::RequestClient},
    models::{
        build_args::BuildArgs, empty_struct::EmptyStruct, errors::RiftInitializationError,
        lcu_runes::LcuRunes, put_rune_page::PutRunePage, rune_page::RunePage,
    },
    utilities::{result_printer, rune_fetcher},
};

pub async fn set_champion_settings(build_args: &BuildArgs) {
    println!("Fetching runes for {}...", build_args.champion);
    let mut champion_settings = rune_fetcher::scrape_champion_settings(build_args).await;

    result_printer::print_result(&champion_settings);
    let lcu_runes = super::runes_mapper::map_champion_settings(&mut champion_settings);
    let mut league_client = get_league_client().unwrap();
    let runes = league_client
        .get::<Vec<RunePage>>(constants::lcu_urls::GET_RUNE_PAGES)
        .await
        .unwrap();

    let put_runes_request_body =
        get_put_runes_request(build_args.champion.as_str(), &lcu_runes).await;

    let first_rune_id = runes[0].id.to_string();
    let first_rune_id_str = first_rune_id.as_str();
    let url = format!("lol-perks/v1/pages/{}", first_rune_id_str);
    league_client
        .put::<EmptyStruct, PutRunePage>(&url, Some(put_runes_request_body))
        .await
        .unwrap_err();
}

async fn get_put_runes_request(champion: &str, lcu_runes: &LcuRunes) -> PutRunePage {
    let runes_refored = http::ddragon_client::get_runes_reforged().await;
    PutRunePage {
        name: format!("Kingens {champion}"),
        current: true,
        primary_style_id: super::runes_mapper::map_runes(
            lcu_runes.primary.to_owned(),
            runes_refored.to_owned(),
        ),
        selected_perk_ids: super::runes_mapper::map_slots(
            lcu_runes.perks.to_owned(),
            runes_refored.to_owned(),
        ),
        sub_style_id: super::runes_mapper::map_runes(lcu_runes.secondary.to_owned(), runes_refored),
    }
}

fn get_league_client() -> Result<RequestClient, RiftInitializationError> {
    let lockfile = super::lockfile_fetcher::get_lockfile();
    match lockfile {
        Ok(l) => Ok(http::league_client_api::get_request_client(l)),
        Err(e) => return Err(e),
    }
}
