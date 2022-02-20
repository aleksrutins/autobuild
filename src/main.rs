mod format;
mod steps;
mod console;
mod config;
mod fs_utils;

use crate::steps::run_all;
use clap::{Parser, Subcommand};
use config::get_installed_recipes;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    requirement: String,

    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand, Debug)]
enum Commands {
    Default {
        recipe: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => {
            match &command {
                Commands::Default { recipe } => {

                }
            }
        },
        None => {
            let recipes = get_installed_recipes();
            if recipes.contains_key(&cli.requirement) {
                console::log_info(&format!("Installed recipes for requirement {}:", cli.requirement.as_str()));
                let matching_recipes = recipes.get(&cli.requirement).unwrap();
                for recipe in matching_recipes {
                    console::log_info(&format!("({:?}) {}/{}", matching_recipes.iter().position(|r| &r.name == &recipe.name).unwrap_or(1) + 1, recipe.author, recipe.name));
                }
                if let Ok(response) = console::question("Which one should I run?").parse::<usize>() {
                    if let Some(recipe) = matching_recipes.get(response - 1) {
                        run_all(&recipe.steps);
                    } else {
                        console::log_err("Please enter a valid index.");
                        return
                    }
                } else {
                    console::log_err("Please enter a number.");
                    return
                }
                
            } else {
                console::log_info(&format!("No recipes installed matching requirement {}.", cli.requirement.as_str()));
            }
        }
    }
}
