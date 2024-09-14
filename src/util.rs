
use crate::cli::{Cli, Operations};
use std::io::{self, Write};

pub fn confirm_prompt(prompt: &str) -> bool {
    let mut input = String::new();
    
    loop {
        print!("{} (Y/N): ", prompt);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut input = input.trim().to_uppercase();
        
        match input.as_str() {
            "Y" | "YES" => return true,
            "N" | "NO" => return false,
            "" => {
                println!("Please enter 'Y' or 'N'.");
                input.clear();
            },
            _ => {
                println!("Invalid input. Please enter 'Y' or 'N'.");
                input.clear();
            }
        }
    }
}

pub fn cmd_handler(cli: &Cli) {  // Note the use of `&Cli`
    match &cli.operation {
        Some(Operations::Install(install)) => {
            if !install.force && !confirm_prompt("Are you sure you want to install the packages") {
                println!("Installation aborted.");
                return;
            }
            println!("Installing packages: {:?}", install.pkgs);
        },
        Some(Operations::Remove(remove)) => {
            if !remove.force && !remove.yes && !confirm_prompt("Are you sure you want to remove the packages") {
                println!("Removal aborted.");
                return;
            }
            println!("Removing packages: {:?}", remove.pkgs);
        },
        Some(Operations::Downgrade(downgrade)) => {
            if !downgrade.force && !confirm_prompt("Are you sure you want to downgrade the packages") {
                println!("Downgrade aborted.");
                return;
            }
            println!("Downgrading packages: {:?}", downgrade.pkgs);
            if let Some(version) = &downgrade.version {
                println!("To version: {}", version);
            }
        },
        Some(Operations::AddRepo(add_repo)) => {
            if !add_repo.update && !confirm_prompt("Are you sure you want to add this repository") {
                println!("Repository addition aborted.");
                return;
            }
            println!("Adding repository: {}", add_repo.repo);
        },
        Some(Operations::Resume(resume)) => {
            if !resume.all && !confirm_prompt("Are you sure you want to resume the operation") {
                println!("Resume aborted.");
                return;
            }
            println!("Resuming operations with ID: {:?}", resume.id);
        },
        _ => (),
    }
}
