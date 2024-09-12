pub mod cli;
pub mod log;
use crate::cli::Cli;
use crate::cli::Operations;
use colored::Colorize;
use clap::Parser;


const VERSION_TEXT: &str = r#"
d8b       d8b                    Iris Package Manager         
Y8P       Y8P                 -------------------------
                      
888888d888888.d8888b          Iris v1.0.0
888888P"  88888K              Copyright (C) 2024-2025 RadianOS Development Team
888888    888"Y8888b.         Copyright (C) 2024-2025 Atiksh Sharma
888888    888     X88         This program may be freely redistributed under
888888    888 88888P'          the terms of the GNU General Public License.
"#;

fn print_version() {
    println!("{}", VERSION_TEXT.bright_cyan().bold());
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        print_version();
        return;
    }

    if let Some(operation) = cli.operation {
        match operation {
            Operations::Install(install) => {
                println!("Installing packages: {:?}", install.pkgs);
            }
            Operations::Remove(remove) => {
                println!("Removing packages: {:?}", remove.pkgs);
            }
            Operations::Search(search) => {
                println!("Searching packages: {:?}", search.terms);
            }
            Operations::Query(query) => {
                println!("Querying packages: {:?}", query.terms);
            }
            Operations::List => {
                println!("Listing packages");
            }
            Operations::Upgrade => {
                println!("Upgrading system packages");
            }
            Operations::Sync => {
                println!("Syncing repositories");
            }
            Operations::AddRepo(add_repo) => {
                println!("Adding repository: {}", add_repo.repo);
            }
        }
    } else {
        println!("No operation specified.");
    }
}
