use crate::utils::{color_print, guard_toybox_pdxinfo_present, parse_game_name_from_toybox_pdxinfo};
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;
use which::which;


pub(crate) fn build_project() -> Result<(), Report> {
    let workdir = guard_toybox_pdxinfo_present()?;
    let workdir = workdir.display();

    dbg!(&workdir);
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

    let pdc_path = which("pdc") .with_section(move || {
        "The compiler could not be found. It is the program used to prepare code into games that can be played."
            .header("Explanation:".yellow())
    })
    .with_section(move || {
        format!(
            "Try installing the Playdate SDK from: {}\nMake sure that the SDK is configured correctly, and you can run the command \"pdc\" in your terminal.",
            "https://play.date/dev/".green()
        )
        .header("Suggestions:".green())
    })?;

    let pdc_output = std::process::Command::new(pdc_path)
        .arg(format!("{workdir}/source"))
        .arg(format!("{workdir}/target/{project_name}"))
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
    color_print(&format!("Successfully built {project_name}"), None);
    Ok(())
}
