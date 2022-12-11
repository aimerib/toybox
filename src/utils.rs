use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::{OwoColorize, Stream::Stdout};

pub(crate) fn color_print<S: AsRef<str> + std::fmt::Display>(message: S, extra_message: Option<S>) {
    println!("\n");

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
    println!("\n");
}

pub(crate) fn guard_toybox_toml_present() -> Result<(), Report> {
    let toybox_toml_path = std::path::PathBuf::from("Toybox.toml");
    if !toybox_toml_path.exists() {
        return Err(eyre!("Toybox.toml not found.")).with_section(move || {
            "Please run \"toybox new\" to create a new project.".header("Suggestion:".yellow())
        });
    }
    Ok(())
}

pub(crate) fn parse_game_name_from_toybox_toml() -> Result<String, Report> {
    let toybox_toml = std::fs::read_to_string("Toybox.toml").unwrap();
    let toml: toml::Value = toml::from_str(&toybox_toml).unwrap();
    let project_name = toml["game"]["name"].as_str().unwrap();
    Ok(project_name.to_string())
}
