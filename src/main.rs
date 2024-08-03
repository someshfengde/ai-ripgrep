mod config;
mod llm;
mod utils;

use config::LLMConfig;
use llm::provider::LLMProvider;
use utils::cli::get_cli_matches;

fn main() {
    let matches = get_cli_matches();
    //  check if the reset arg is given
    if matches.get_flag("reset") {
        let _ = config::LLMConfig::reset_config();
        println!(
            "Your configuration is reseted.\n New configuration can be done with `arg --configure`"
        )
    }

    if let Some(config_values) = matches.get_many::<String>("configure") {
        let config_values: Vec<&String> = config_values.collect();
        let config = LLMConfig::new(
            &config_values[0],
            &config_values[2],
            Some(&config_values[1]),
        );
        config.save_to_file().expect("Failed to save configuration");
        println!("Configuration saved successfully.");
    }

    if let Some(query) = matches.get_one::<String>("query") {
        match LLMConfig::load_from_file() {
            Ok(config) => {
                let provider = LLMProvider::new(&config);
                let response = provider.query(query).expect("Failed to query the LLM");
                println!("Response: {}", response);
            }
            Err(_) => {
                println!("No configuration found. Please configure using --configure first.");
            }
        }
    }
}
