use crate::utils::{color_print, guard_toybox_pdxinfo_present, parse_game_name_from_toybox_pdxinfo};
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;
use which::which;


pub(crate) fn build_project() -> Result<(), Report> {
    guard_toybox_pdxinfo_present()?;

    let project_name = parse_game_name_from_toybox_pdxinfo()?;

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

    // let pdc_output =

    std::process::Command::new("which")
        .arg("pdc")
        .output()
        .expect("failed to execute process");

    let pdc_path = which("pdc").with_section(move || {
        "pdc is the compiler used to compile Toybox projects."
            .header("Explanation:".yellow())
    })
    .with_section(move || {
        format!(
            "Try installing pdc using \"{}\".",
            "cargo install pdc".green()
        )
        .header("Suggestions:".green())
    })?;

    // if !pdc_output.status.success() {
    //     return Err(eyre!("pdc is not installed"))
    //         .with_section(move || {
    //             "pdc is the compiler used to compile Toybox projects."
    //                 .header("Explanation:".yellow())
    //         })
    //         .with_section(move || {
    //             format!(
    //                 "Try installing pdc using \"{}\".",
    //                 "cargo install pdc".green()
    //             )
    //             .header("Suggestions:".green())
    //         });
    // }

    let pdc_output = std::process::Command::new(pdc_path)
        .arg("source")
        .arg(format!("target/{project_name}"))
        .output()
        .expect("failed to execute process");

    if !pdc_output.status.success() {
        return Err(eyre!("pdc exited with error"))
            .with_section(move || {
                "pdc is the compiler used to compile Toybox projects."
                    .header("Explanation:".yellow())
            })
            .with_section(move || {
                format!(
                    "Try installing pdc using \"{}\".",
                    "cargo install pdc".green()
                )
                .header("Suggestions:".green())
            });
    }
    color_print(format!("Successfully built {project_name}"), None);
    Ok(())
}
