use std::{
    fs::rename,
    path::{Path, PathBuf},
};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "bak",
    about = "Append a tilde (~) to the names of given files/directories."
)]
struct Options {
    #[structopt(
        help = "The file(s)/director(y/ies) to append tilde.",
        parse(from_os_str)
    )]
    sources: Vec<PathBuf>,
    #[structopt(long = "--quiet", short = "-q", help = "Be quiet.")]
    quiet: bool,
}

fn main() -> Result<(), String> {
    let mut conf = Options::from_args();
    if conf.sources.len() == 0 {
        if let Err(_) = Options::clap().print_help() {
            return Err("Failed printing help message".to_string());
        }
        return Ok(());
    }

    // Check for validity of given source paths.
    let mut invalid_sources: Vec<PathBuf> = Vec::new();
    for source in conf.sources.iter() {
        if !source.is_file() && !source.is_dir() {
            invalid_sources.push(source.to_path_buf());
        }
    }
    if invalid_sources.len() > 0 {
        eprintln!(
            "The following input paths are invalid:\n{:#?}",
            invalid_sources
        );
        eprintln!("Nothing done");
        return Err("Invalid path(s)".to_string());
    }

    // Check for possible confliction of target paths.
    let mut invalid_targets: Vec<PathBuf> = Vec::new();
    for source in conf.sources.iter() {
        let target = Path::new(
            &(source.file_name().unwrap().to_str().unwrap().to_owned() + "~"),
        )
        .to_path_buf();
        if target.exists() {
            invalid_targets.push(source.to_path_buf());
        }
    }
    if invalid_targets.len() > 0 {
        eprintln!(
            "The following paths have existing target paths:\n{:#?}",
            invalid_targets
        );
        eprintln!("Nothing done");
        return Err("Conflicting target path(s)".to_string());
    }

    // Actual renaming process.
    for source in conf.sources.iter_mut() {
        let target = Path::new(
            &(source.file_name().unwrap().to_str().unwrap().to_owned() + "~"),
        )
        .to_path_buf();
        match rename(&source, &target) {
            Ok(()) => {
                if !conf.quiet {
                    println!("{:?} -> {:?}", source, target);
                }
            }
            Err(err) => {
                eprintln!("Error renaming {:?}: {:?}", source, err);
            }
        }
    }

    Ok(())
}
// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jun 29 2021, 19:32 [CST]
