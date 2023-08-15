mod api;
use api::config::load_config;
use api::generate_symlinks::generate_symlinks;
use api::log::Logger;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = "config.json";

    // Load the API configuration from the JSON file
    let api_config = load_config(config_path)?;

    // Generate symlinks for all files in the specified folders
    let mut logger = Logger::new("log.txt").unwrap();
    generate_symlinks(&api_config, &mut logger)?;
    println!("Symlinks generated successfully..");

    Ok(())
}
