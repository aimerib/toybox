use crate::utils::{guard_toybox_pdxinfo_present, parse_game_name_from_toybox_pdxinfo, color_print};
use color_eyre::{eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

use super::build::build_project;

pub(crate) fn run_project() -> Result<(), Report> {
    let workdir = guard_toybox_pdxinfo_present()?;
    let workdir = workdir.display();
    build_project()?;
    let project_name = parse_game_name_from_toybox_pdxinfo()?;
    let file_name = if std::env::consts::OS == "windows" {
        format!("{workdir}\\target\\{project_name}.pdx\\main.pdz")
    } else {
        format!("{workdir}/target/{project_name}.pdx")
    };
    dbg!(&file_name);
    open::that(file_name)
        .with_section(move || {
            "The emulator could not be found. It is the program used to run Toybox projects."
                .header("Explanation:".yellow())
        })
        .with_section(move || {
            format!(
                "Try installing the Playdate SDK from: \"{}\"\nMake sure that the SDK is configured correctly, and you can run the command \"pdc\" in your terminal.",
                "https://play.date/dev/".green()
            )
            .header("Suggestions:".green())
        })?;
    color_print("Running", Some(&project_name));
    Ok(())
}
