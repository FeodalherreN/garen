use log::debug;

use crate::models::build_args::BuildArgs;

pub fn build_url(build_args: &BuildArgs) -> String {
    match &build_args.mode {
        Some(_) => match &build_args.role {
            Some(role) => {
                return format!(
                    "https://u.gg/lol/champions/{}/build?role={}",
                    build_args.champion, role
                );
            }
            None => return format!("https://u.gg/lol/champions/{}/build", build_args.champion),
        },
        None => debug!("No mode selected, defaulting to ARAM."),
    }

    format!(
        "https://u.gg/lol/champions/aram/{}-aram",
        build_args.champion
    )
}
