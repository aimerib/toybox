use crate::utils::{color_print, guard_toybox_toml_present};
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

pub(crate) fn build_project() -> Result<(), Report> {
    guard_toybox_toml_present()?;
    color_print("Building project", None);

    let target_path = std::path::PathBuf::from("target");
    if !target_path.exists() {
        std::fs::create_dir(&target_path).unwrap();
    } else {
        let target_path_read_dir = target_path.read_dir().unwrap();
        let target_path_count = target_path_read_dir.count();
        if target_path_count > 0 {
            std::fs::remove_dir_all("target").unwrap();
            std::fs::create_dir(&target_path).unwrap();
        }
    }    

    let pdc_output = std::process::Command::new("which")
        .arg("pdc")
        .output()
        .expect("failed to execute process");
    if !pdc_output.status.success() {
        return Err(eyre!("pdc is not installed"))
            .with_section(move || "pdc is the compiler used to compile Toybox projects.".header("Explanation:".yellow()))
            .with_section(move || format!("Try installing pdc using \"{}\".", "cargo install pdc".green()).header("Solutions:".green()));
    }

    let pdc_output = std::process::Command::new("pdc")
        .arg("src")
        .arg("target")
        .output()
        .expect("failed to execute process");

        if !pdc_output.status.success() {
        return Err(eyre!("pdc exited with error"))
            .with_section(move || "pdc is the compiler used to compile Toybox projects.".header("Explanation:".yellow()))
            .with_section(move || format!("Try installing pdc using \"{}\".", "cargo install pdc".green()).header("Solutions:".green()));
    }
    Ok(())
}