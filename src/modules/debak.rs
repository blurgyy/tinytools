use std::{fs::rename, path::PathBuf, str::FromStr};

fn get_target(path: PathBuf) -> Result<PathBuf, String> {
    let mut full_path = path.to_str().unwrap().to_owned();
    if full_path.ends_with("~") {
        full_path.pop();
        Ok(PathBuf::from_str(&full_path).unwrap())
    } else {
        Err(format!(
            "Path <{}> is not a backup file (because it has no trailing tilde)",
            path.to_str().unwrap()
        ))
    }
}

pub fn debak(
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
        .map(|source| super::_shared_functions::normalize_path(source))
        .collect();

    // Check for possible confliction of target paths.
    let mut invalid_targets: Vec<PathBuf> = Vec::new();
    for source in sources.iter() {
        let target = get_target(source.to_path_buf())?;
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
        let target = get_target(source.to_path_buf())?;
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
// Date:   Jun 30 2021, 21:35 [CST]
