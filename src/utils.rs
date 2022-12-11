use std::path::{PathBuf, Path};

use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::{OwoColorize, Stream::Stdout};

pub(crate) fn color_print(
    message: &str,
    extra_message: Option<&str>,
)
{
    if let Some(extra_message) = extra_message {
        println!(
            "{}{} {}",
            message.if_supports_color(Stdout, |text| text.bright_blue()),
            ":".if_supports_color(Stdout, |text| text.bright_blue()),
            extra_message.if_supports_color(Stdout, |text| text.bright_green())
        );
    } else {
        println!(
            "{}",
            message.if_supports_color(Stdout, |text| text.bright_green())
        );
    }
}

pub(crate) fn guard_toybox_pdxinfo_present() -> Result<PathBuf, Report> {
    // let toybox_pdxinfo_path = std::path::PathBuf::from("pdxinfo");
    let mut path: PathBuf = std::env::current_dir()?;
    let file = Path::new("pdxinfo");

    loop {
        path.push(file);

        if path.is_file() {
            if let Some(workdir) =  &path.parent(){
                break Ok(workdir.into());
            };
            break Err(eyre!("pdxinfo not found.")).with_section(move || {
                "Please run \"toybox new\" to create a new project.".header("Suggestion:".yellow())
            });
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break Err(eyre!("pdxinfo not found.")).with_section(move || {
                        "Please run \"toybox new\" to create a new project.".header("Suggestion:".yellow())
                    });
        }
    }
    // if !toybox_pdxinfo_path.exists() {
    //     // recursively search for pdxinfo file in parent directories
    //     // if not found, return error


    //     return Err(eyre!("pdxinfo not found.")).with_section(move || {
    //         "Please run \"toybox new\" to create a new project.".header("Suggestion:".yellow())
    //     });
    // }
    // Ok(())
}

pub(crate) fn parse_game_name_from_toybox_pdxinfo() -> Result<String, Report> {
    let toybox_pdxinfo = std::fs::read_to_string("pdxinfo").unwrap();
    let lines = toybox_pdxinfo.lines();
    for line in lines {
        if line.starts_with("name") {
            let project_name = line.split('=').collect::<Vec<&str>>()[1].trim();
            return Ok(project_name.to_string());
        }
    }
    Err(eyre!("Could not parse game name from pdxinfo."))
        .with_section(move || "Please check the pdxinfo file.".header("Suggestion:".yellow()))
}
