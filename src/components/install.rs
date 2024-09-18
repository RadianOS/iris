use std::fs::File;

use std::io::{
    self,
    Result as IoResult
};
use reqwest::blocking::get;
use tar::Archive;
use xz2::read::XzDecoder;

use indicatif::{
    ProgressBar, 
    ProgressStyle
};
use std::io::Read;
use crate::confirm;

pub fn download_package(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(output_path)?;
    io::copy(&mut response.take(usize::MAX as u64), &mut file)?;
    Ok(())
}

pub fn extract_package(tar_xz_path: &str, output_dir: &str) -> IoResult<()> {
    let file = File::open(tar_xz_path)?;
    let decompressor = XzDecoder::new(file);
    let mut archive = Archive::new(decompressor);
    archive.unpack(output_dir)?;
    Ok(())
}

pub fn install_packages(pkgs: &[String], force: bool) {
    if !force && !confirm("Are you sure you want to install the packages") {
        println!("Installation aborted.");
        return;
    }
    
    let total = pkgs.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {wide_bar} {pos}/{len} ({eta})").expect("{}")
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
