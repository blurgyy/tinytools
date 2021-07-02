use std::{
    fs::{canonicalize, rename},
    path::{Path, PathBuf},
};

fn get_target(path: PathBuf) -> PathBuf {
    return Path::new(&(path.to_str().unwrap().to_owned() + "~"))
        .to_path_buf();
}

pub fn bak(
    source: PathBuf,
    more_sources: Vec<PathBuf>,
    quiet: bool,
) -> Result<(), String> {
    let sources = &mut vec![source];
    sources.extend(more_sources);

    // Validate input paths.
    super::_shared_functions::validate_paths(sources.to_vec())?;

    // Canonicalize to absolute paths.
    *sources = sources
        .iter()
        .map(|source| canonicalize(source).unwrap())
        .collect();

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
