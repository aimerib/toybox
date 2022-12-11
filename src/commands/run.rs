use crate::utils::{guard_toybox_toml_present, parse_game_name_from_toybox_toml};
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

use super::build::build_project;

pub(crate) fn run_project() -> Result<(), Report> {
    guard_toybox_toml_present()?;
    build_project()?;
    let project_name = parse_game_name_from_toybox_toml()?;
    let emulator_output = std::process::Command::new("open")
        .arg(format!("target/{project_name}.pdx"))
        .output()
        .expect("failed to execute process");
    if !emulator_output.status.success() {
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
