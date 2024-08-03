use clap::{Arg, ArgAction, Command};

pub fn get_cli_matches() -> clap::ArgMatches {
    Command::new("ai-ripgrep")
        .version("1.0")
        .author("Someshfengde")
        .about("Searching with LLMs for ripgrep.")
        .arg(
            Arg::new("configure")
                .long("configure")
                .num_args(3)
                .value_names(&["PROVIDER", "API_KEY", "MODEL"])
                .help("Configure the LLM provider"),
        )
        .arg(
            Arg::new("reset")
                .long("reset")
                .action(ArgAction::SetTrue)
                .help("Reset the configuration files."),
        )
        .arg(
            Arg::new("query")
                .help("Query to be processed by the configured LLM")
                .required_unless_present_any(&["configure", "reset"])
                .index(1),
        )
        .get_matches()
}
