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
    pub static ref CONFIG_DIR: String = dirs::config_dir().expect("Could not get config dir").to_str().expect("Could not convert path to string").to_string() + "/autobuild/recipes";
}

pub fn get_installed_recipes() -> HashMap<String, Vec<Recipe>> {
    let dir = fs::read_dir(CONFIG_DIR.as_str());
    match dir {
        Ok(_) => {
            let mut result: HashMap<String, Vec<Recipe>> = HashMap::new();
            for entry in WalkDir::new(CONFIG_DIR.as_str()).follow_links(true).into_iter().filter_map(|entry| entry.ok()) {
                if entry.file_type().is_file() {
                    match File::open(entry.path()) {
                        Ok(mut file) => {
                            if let Ok(content) = read_file(&mut file) {
                                if let Ok(recipe) = ron::from_str::<Recipe>(content.as_str()) {
                                    if result.contains_key(&recipe.provides) {
                                        if let Some(mut entry) = result.get_mut(&recipe.provides) {
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
            console::log_info("No config directory found; creating");
            fs::create_dir_all(CONFIG_DIR.as_str());
            HashMap::new()
        }
    }
}