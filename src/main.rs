use std::env;

mod constants;
mod http;
mod models;
mod utilities;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let champion = args[1].as_str();
        utilities::rune_setter::set_runes(champion).await;
    } else {
        println!("No champion passed as argument");
    }
}
