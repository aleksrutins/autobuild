mod format;
mod steps;
mod console;
mod config;
mod fs_utils;

use std::{fs::File, io::{Error, ErrorKind, Write}};

use crate::steps::run_all;
use clap::{Parser, Subcommand};
use config::{get_installed_recipes, get_user_config};
use ron::ser::PrettyConfig;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        requirement: String,
    },
    Default {
        recipe: String
    },
    Config,
    GenConfig
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let config = get_user_config();

    match &cli.command {
        Commands::Default { recipe } => {
            if let Some(default) = config.defaults.get(recipe) {
                println!("{}", default);
            } else {
                console::log_info(&format!("No default set for {}", recipe));
            }
        },
        Commands::Config => {
            println!("{}", config::CONFIG_DIR.to_string());
        },
        Commands::GenConfig => {
            if let Ok(_) = File::open(config::config_dir("config.ron")) {
                let response = console::question("The user configuration file already exists; overwrite? [yN]");
                if response != "y" {
                    return Ok(())
                }
            }
            let mut file = File::create(config::config_dir("config.ron"))?;
            file.write_all(ron::ser::to_string_pretty(&(*config::DEFAULT_CONFIG), PrettyConfig::new()).expect("Could not serialize configuration").as_bytes())?;
        },
        Commands::Run { requirement } => {
            let recipes = get_installed_recipes();
            if recipes.contains_key(requirement) {
                console::log_info(&format!("Installed recipes for requirement {}:", requirement.as_str()));
                let matching_recipes = recipes.get(requirement).unwrap();
                for recipe in matching_recipes {
                    console::log_info(&format!("({:?}) {}/{}", matching_recipes.iter().position(|r| &r.name == &recipe.name).unwrap_or(1) + 1, recipe.author, recipe.name));
                }
                if let Ok(response) = console::question("Which one should I run?").parse::<usize>() {
                    if let Some(recipe) = matching_recipes.get(response - 1) {
                        run_all(&recipe.steps);
                    } else {
                        console::log_err("Please enter a valid index.");
                        return Err(Error::new(ErrorKind::Other, "Please enter a valid index."));
                    }
                } else {
                    console::log_err("Please enter a number.");
                    return Err(Error::new(ErrorKind::Other, "Please enter a number."));
                }
                
            } else {
                console::log_info(&format!("No recipes installed matching requirement {}.", requirement));
            }
        }
    }
    Ok(())
}
