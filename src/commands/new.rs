use crate::utils::color_print;
use color_eyre::{eyre::eyre, eyre::Report, Section, SectionExt};
use owo_colors::OwoColorize;

pub(crate) fn new_project(name: Option<String>) -> Result<(), Report> {
    let main_lua = include_str!("../../templates/main.lua.template");
    let name = match name {
        Some(name) => name,
        None => {
            let current_dir = std::env::current_dir().unwrap();
            let current_dir = current_dir.file_name().unwrap();
            let current_dir = current_dir.to_str().unwrap();
            current_dir.to_string()
        }
    };

    color_print("Creating new project", Some(&name));

    let project_path = std::path::PathBuf::from(&name);

    if project_path == std::path::PathBuf::from(".") {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir = current_dir.read_dir().unwrap();
        let current_dir = current_dir.count();
        if current_dir > 0 {
            return Err(eyre!("Current directory is not empty"))
                .with_section(move || "A new project should only be created in an empty directory.".header("Explanation:".yellow()))
                .with_section(move || format!("Try using a different name, removing the existing directory {}, or using \"{}\" instead.", name.red(), "toybox init".green()).header("Solutions:".green()));
        }
    }

    if project_path.exists() {
        return Err(eyre!("Current directory is not empty"))
            .with_section(move || "A new project should only be created in an empty directory.".header("Explanation:".yellow()))
            .with_section(move || format!("Try using a different name, removing the existing directory {}, or using \"{}\" instead.", name.red(), "toybox init".green()).header("Solutions:".green()));
    }

    std::fs::create_dir(&project_path).unwrap();

    std::fs::create_dir(project_path.join("src")).unwrap();

    std::fs::write(project_path.join("src/main.lua"), main_lua).unwrap();

    let toybox_toml = include_str!("../../templates/Toybox.toml.template");
    let toybox_toml = toybox_toml.replace("{{name}}", &format!("\"{name}\""));
    std::fs::write(project_path.join("Toybox.toml"), toybox_toml).unwrap();

    color_print("All done!", None);

    Ok(())
}