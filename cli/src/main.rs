use clap::Parser;
use cli::custom::custom_menu;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Software updater CLI")]
struct CLI {
    path: Option<PathBuf>,
}

fn main() {
    CLI::parse();

    custom_menu();
}
