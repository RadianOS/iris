use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "",
    version = "1.0",
    about = "The Iris Package Manager",
    long_about = None,
    disable_version_flag = true // Disable default version flag
)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Option<Operations>,

    // Define a custom version flag
    #[arg(short = 'V', long = "version", help = "Print version information")]
    pub version: bool,
}

#[derive(Debug, Subcommand)]
pub enum Operations {
    Install(Install),
    Remove(Remove),
    Search(Search),
    Query(Query),
    List,
    Upgrade,
    Sync,
    AddRepo(AddRepo),
}

#[derive(Parser, Debug)]
pub struct Install {
    #[arg(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Remove {
    #[arg(index = 1)]
    pub pkgs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Search {
    #[arg(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Query {
    #[arg(index = 1)]
    pub terms: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct AddRepo {
    #[arg(index = 1)]
    pub repo: String,
}
