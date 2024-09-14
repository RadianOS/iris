use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "",
    version = "1.0",
    about = "The Iris Package Manager",
    long_about = None,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Option<Operations>,
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
    Downgrade(Downgrade),
    Resume(Resume),
}

#[derive(Parser, Debug)]
pub struct Install {
    #[arg(short = 'p', long = "package", help = "Specify package(s) to install")]
    pub pkgs: Vec<String>,

    #[arg(short = 'f', long = "force", help = "Force installation")]
    pub force: bool,
}

#[derive(Parser, Debug)]
pub struct Remove {
    #[arg(short = 'p', long = "package", help = "Specify package(s) to remove")]
    pub pkgs: Vec<String>,

    #[arg(short = 'y', long = "yes", help = "Automatically answer yes to prompts")]
    pub yes: bool,
}

#[derive(Parser, Debug)]
pub struct Search {
    #[arg(short = 't', long = "term", help = "Specify search term(s)")]
    pub terms: Vec<String>,

    #[arg(short = 'a', long = "all", help = "Search all available packages")]
    pub all: bool,
}

#[derive(Parser, Debug)]
pub struct Query {
    #[arg(short = 't', long = "term", help = "Specify query term(s)")]
    pub terms: Vec<String>,

    #[arg(short = 'd', long = "details", help = "Show detailed information")]
    pub details: bool,
}

#[derive(Parser, Debug)]
pub struct AddRepo {
    #[arg(short = 'r', long = "repo", help = "Specify repository URL")]
    pub repo: String,

    #[arg(short = 'u', long = "update", help = "Update the repository list")]
    pub update: bool,
}

#[derive(Parser, Debug)]
pub struct Downgrade {
    #[arg(short = 'p', long = "package", help = "Specify package(s) to downgrade")]
    pub pkgs: Vec<String>,

    #[arg(short = 'v', long = "version", help = "Specify version to downgrade to")]
    pub version: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Resume {
    #[arg(short = 'a', long = "all", help = "Resume all paused operations")]
    pub all: bool,

    #[arg(short = 'i', long = "id", help = "Specify ID of the operation to resume")]
    pub id: Option<String>,
}
