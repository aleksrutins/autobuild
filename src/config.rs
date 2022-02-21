use std::{collections::HashMap, fs::{self, File}};

use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use lazy_static::lazy_static;

use crate::{format::{Step}, console, fs_utils::read_file};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub author: String,
    pub provides: String,
    pub steps: Vec<Step>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub proxy_url: String,
    pub defaults: HashMap<String, String>
}

lazy_static! {
    pub static ref CONFIG_DIR: String = dirs::config_dir().expect("Could not get config dir").to_str().expect("Could not convert path to string").to_string() + "/autobuild";
    pub static ref DEFAULT_CONFIG: UserConfig = UserConfig {
        proxy_url: "https://autobuild-proxy-production.up.railway.app".to_string(),
        defaults: HashMap::new()
    };
}

pub fn config_dir(subpath: &str) -> String { CONFIG_DIR.to_string() + "/" + subpath }

pub fn get_user_config() -> UserConfig {
    if let Err(_) = fs::read_dir(CONFIG_DIR.to_string()) {
        console::log_info(&format!("Creating configuration directory {}", CONFIG_DIR.to_string()));
        if let Err(_) = fs::create_dir_all(CONFIG_DIR.to_string()) {
            panic!("Could not create configuration directory")
        }
    };
    if let Ok(result) = fs::read(config_dir("config.ron")) {
        match ron::from_str::<UserConfig>(String::from_utf8(result).expect("Could not convert user config to string").as_str()) {
            Ok(config) => config,
            Err(_) => (*DEFAULT_CONFIG).clone()
        }
    } else {
        (*DEFAULT_CONFIG).clone()
    }
}

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
            if let Err(_) = fs::create_dir_all(recipes_dir) {
                console::log_err("Error creating configuration directory");
            }
            HashMap::new()
        }
    }
}