use crate::utils::{guard_toybox_pdxinfo_present, parse_game_name_from_toybox_pdxinfo};
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

use super::build::build_project;

pub(crate) fn run_project() -> Result<(), Report> {
    guard_toybox_pdxinfo_present()?;
    build_project()?;
    let project_name = parse_game_name_from_toybox_pdxinfo()?;
    let emulator_output = open::that(format!("target/{project_name}.pdx"));
    // let emulator_output = std::process::Command::new("open")
    //     .arg(format!("target/{project_name}.pdx"))
    //     .output()
    //     .expect("failed to execute process");
    if !emulator_output.is_err() {
        return Err(eyre!("emulator exited with error"))
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
            });
    }
    Ok(())
}
