use std::{collections::HashMap, fs::{self, File}, hash::Hash, io::{Read, self}, env};

use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use lazy_static::lazy_static;

use crate::{format::{Step, Configuration}, console, fs_utils::read_file};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub author: String,
    pub provides: String,
    pub steps: Vec<Step>
}

lazy_static! {
    pub static ref CONFIG_DIR: String = dirs::config_dir().expect("Could not get config dir").to_str().expect("Could not convert path to string").to_string() + "/autobuild";
}

pub fn config_dir(subdir: &str) -> String { CONFIG_DIR.to_string() + "/" + subdir }

pub fn get_installed_recipes() -> HashMap<String, Vec<Recipe>> {
    let recipes_dir = &config_dir("recipes");
    let dir = fs::read_dir(recipes_dir);
    match dir {
        Ok(_) => {
            let mut result: HashMap<String, Vec<Recipe>> = HashMap::new();
            for entry in WalkDir::new(recipes_dir).follow_links(true).into_iter().filter_map(|entry| entry.ok()) {
                if entry.file_type().is_file() {
                    match File::open(entry.path()) {
                        Ok(mut file) => {
                            if let Ok(content) = read_file(&mut file) {
                                if let Ok(recipe) = ron::from_str::<Recipe>(content.as_str()) {
                                    if result.contains_key(&recipe.provides) {
                                        if let Some(entry) = result.get_mut(&recipe.provides) {
                                            entry.append(&mut vec![recipe]);
                                        }
                                    } else {
                                        result.insert(recipe.provides.clone(), vec![recipe]);
                                    }
                                } else {
                                    console::log_err(&format!("(non-fatal) Malformed recipe {}", entry.file_name().to_str().expect("Could not convert file name to string")));
                                }
                            }
                        },
                        Err(_) => {
                            console::log_err(&format!("Error opening config file {} (non-fatal)", entry.file_name().to_str().expect("Could not convert file name to string")).to_string())
                        }
                    }
                }
            }
            result
        }
        Err(_) => {
            console::log_info(&format!("Creating configuration directory {}", recipes_dir));
            fs::create_dir_all(recipes_dir);
            HashMap::new()
        }
    }
}