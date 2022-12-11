use crate::utils::{guard_toybox_pdxinfo_present, parse_game_name_from_toybox_pdxinfo, color_print};
use color_eyre::{eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

use super::build::build_project;

pub(crate) fn run_project() -> Result<(), Report> {
    guard_toybox_pdxinfo_present()?;
    build_project()?;
    let project_name = parse_game_name_from_toybox_pdxinfo()?;
    open::that(format!("target/{project_name}.pdx"))
        .with_section(move || {
            "The emulator is the program used to run Toybox projects."
                .header("Explanation:".yellow())
        })
        .with_section(move || {
            format!(
                "Try installing the emulator using \"{}\".",
                "cargo install emulator".green()
            )
            .header("Suggestions:".green())
        })?;
    color_print("Running", project_name);
    Ok(())
}
