use crate::{
    constants,
    http::{self, league_client_api::RequestClient},
    models::{
        errors::RiftInitializationError, lcu_runes::LcuRunes, put_rune_page::PutRunePage,
        put_rune_page_response::PutRunePageResponse, rune_page::RunePage,
    },
    utilities::rune_fetcher,
};

pub async fn set_runes(champion: &str) {
    println!("Fetching runes for {}...", champion);
    let lcu_runes = rune_fetcher::scrape_runes(champion).await;

    println!("Settings runes for {}...", champion);
    let mut league_client = get_league_client().unwrap();
    let runes = league_client
        .get::<Vec<RunePage>>(constants::lcu_urls::GET_RUNE_PAGES)
        .await
        .unwrap();

    let put_runes_request_body = get_put_runes_request(champion, &lcu_runes).await;
    println!("{:?}", put_runes_request_body);

    let first_rune_id = runes[0].id.to_string();
    let first_rune_id_str = first_rune_id.as_str();
    let url = format!("lol-perks/v1/pages/{}", first_rune_id_str);
    league_client
        .put::<PutRunePageResponse, PutRunePage>(&url, Some(put_runes_request_body))
        .await
        .unwrap_err();

    println!("{:?}", lcu_runes);
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
