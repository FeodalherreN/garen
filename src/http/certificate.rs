use crate::constants::certificate::RIOT_GAMES_CERT;
use reqwest::Certificate;

pub fn get_certificate() -> Certificate {
    return Certificate::from_pem(RIOT_GAMES_CERT.as_ref()).unwrap();
}
