use std::{
    fs::{canonicalize, rename},
    path::{Path, PathBuf},
};

fn get_target(path: PathBuf) -> PathBuf {
    return Path::new(&(path.to_str().unwrap().to_owned() + "~"))
        .to_path_buf();
}

pub fn bak(sources: &mut Vec<PathBuf>, quiet: bool) -> Result<(), String> {
    // Abort on empty input.
    if sources.len() == 0 {
        return Err("Missing input path(s)".to_string());
    }

    // Check for validity of given source paths.
    let mut invalid_sources: Vec<PathBuf> = Vec::new();
    for source in sources.iter() {
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

    // Canonicalize to absolute paths.
    for source in sources.iter_mut() {
        *source = canonicalize(&source).unwrap();
    }

    // Check for possible confliction of target paths.
    let mut invalid_targets: Vec<PathBuf> = Vec::new();
    for source in sources.iter() {
        let target = get_target(source.to_path_buf());
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
    for source in sources.iter_mut() {
        let target = get_target(source.to_path_buf());
        match rename(&source, &target) {
            Ok(()) => {
                if !quiet {
                    println!("renamed {:?} -> {:?}", source, target);
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
