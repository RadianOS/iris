pub mod cli;
pub mod log;
pub mod util;

use crate::cli::{Cli, Operations};
use colored::Colorize;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::fs::File;
use std::io::{self, Write, Read, Result as IoResult};
use tar::Archive;
use xz2::read::XzDecoder;
use util::cmd_handler;

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

fn confirm(prompt: &str) -> bool {
    let mut input = String::new();
    
    loop {
        print!("{} (Y/N): ", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut input = input.trim().to_uppercase();
        
        match input.as_str() {
            "Y" | "YES" => return true,
            "N" | "NO" => return false,
            _ => {
                println!("Invalid input. Please enter 'Y' or 'N'.");
                input.clear();
            }
        }
    }
}

fn download_package(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(output_path)?;
    io::copy(&mut response.take(usize::MAX as u64), &mut file)?;
    Ok(())
}

fn extract_package(tar_xz_path: &str, output_dir: &str) -> IoResult<()> {
    let file = File::open(tar_xz_path)?;
    let decompressor = XzDecoder::new(file);
    let mut archive = Archive::new(decompressor);
    archive.unpack(output_dir)?;
    Ok(())
}

fn install_packages(pkgs: &[String], force: bool) {
    if !force && !confirm("Are you sure you want to install the packages") {
        println!("Installation aborted.");
        return;
    }
    
    let total = pkgs.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {wide_bar} {pos}/{len} ({eta})").expect("REASON")
        .progress_chars("#>-"));

    for pkg in pkgs {
        let url = format!("https://raw.githubusercontent.com/RadianOS/zephpkgs/main/{}", pkg);
        let output_path = format!("/tmp/{}", pkg);

        if let Err(e) = download_package(&url, &output_path) {
            eprintln!("Failed to download {}: {}", pkg, e);
            continue;
        }

        if let Err(e) = extract_package(&output_path, "/home/rudy") {
            eprintln!("Failed to extract {}: {}", pkg, e);
        } else {
            println!("Successfully installed {}", pkg);
        }

        pb.inc(1);
    }
    pb.finish_with_message("Installation complete");
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        print_version();
        return;
    }

    cmd_handler(&cli);

    if let Some(operation) = cli.operation {
        match operation {
            Operations::Install(install) => {
                install_packages(&install.pkgs, install.force);
            }
            Operations::Remove(remove) => {
                if !remove.force && !remove.yes && !confirm("Are you sure you want to remove the packages") {
                    println!("Removal aborted.");
                    return;
                }
                println!("Removing packages: {:?}", remove.pkgs);
                if remove.force {
                    println!("Force removal enabled.");
                }
            }
            Operations::Search(search) => {
                println!("Searching packages: {:?}", search.terms);
                if search.all {
                    println!("Searching all available packages.");
                }
            }
            Operations::Query(query) => {
                println!("Querying packages: {:?}", query.terms);
                if query.details {
                    println!("Detailed information requested.");
                }
            }
            Operations::List => {
                println!("Listing packages");
            }
            Operations::Upgrade => {
                if !confirm("Are you sure you want to upgrade the system packages") {
                    println!("Upgrade aborted.");
                    return;
                }
                println!("Upgrading system packages");
            }
            Operations::Sync => {
                if !confirm("Are you sure you want to sync the repositories") {
                    println!("Sync aborted.");
                    return;
                }
                println!("Syncing repositories");
            }
            Operations::AddRepo(add_repo) => {
                println!("Adding repository: {}", add_repo.repo);
                if add_repo.update {
                    if !confirm("Are you sure you want to update the repository list") {
                        println!("Update aborted.");
                        return;
                    }
                    println!("Updating repository list.");
                }
            }
            Operations::Downgrade(downgrade) => {
                if !downgrade.force && !confirm("Are you sure you want to downgrade the packages") {
                    println!("Downgrade aborted.");
                    return;
                }
                println!("Downgrading packages: {:?}", downgrade.pkgs);
                if let Some(version) = downgrade.version {
                    println!("Downgrading to version: {}", version);
                }
            }
            Operations::Resume(resume) => {
                if !resume.all && resume.id.is_none() && !confirm("Are you sure you want to resume the operation") {
                    println!("Resume aborted.");
                    return;
                }
                if resume.all {
                    println!("Resuming all paused operations.");
                } else if let Some(id) = resume.id {
                    println!("Resuming operation with ID: {}", id);
                } else {
                    println!("Resuming operation");
                }
            }
        }
    } else {
        println!("No operation specified.");
    }
}
