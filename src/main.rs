use clap::{arg, Command};
use log::debug;
use models::build_args::BuildArgs;

mod constants;
mod http;
mod models;
mod utilities;

#[macro_use]
extern crate prettytable;

#[tokio::main]
async fn main() {
    let matches = Command::new("build")
        .about("A champion meta builder")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("champion")
                .about("The name of the champion you are going to play.")
                .arg(arg!(<CHAMPION> "Champion name"))
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("mode")
                        .about("The mode that you are going to play (ARAM/NORMAL)")
                        .arg(arg!(<MODE> "The mode you are going to play."))
                        .arg_required_else_help(true)
                        .subcommand(
                            Command::new("role")
                                .about("The specific role you will play.")
                                .arg(arg!(<ROLE> "The role you will play"))
                                .arg_required_else_help(true),
                        ),
                ),
        )
        .get_matches();

    let mut build_args = BuildArgs::new();

    match matches.subcommand() {
        Some(("champion", sub_matches)) => {
            build_args.champion = sub_matches.value_of("CHAMPION").unwrap().to_string();

            match sub_matches.subcommand() {
                Some(("mode", mode_sub_matches)) => {
                    build_args.mode = Some(mode_sub_matches.value_of("MODE").unwrap().to_string());

                    match mode_sub_matches.subcommand() {
                        Some(("role", role_sub_matches)) => {
                            build_args.role =
                                Some(role_sub_matches.value_of("ROLE").unwrap().to_string());
                        }
                        None => debug!("No role found"),
                        _ => unreachable!(),
                    }
                }
                None => debug!("No mode found"),
                _ => unreachable!(),
            }
        }
        None => debug!("No champion found"),
        _ => unreachable!(),
    }

    utilities::rune_setter::set_champion_settings(&build_args).await;
}
