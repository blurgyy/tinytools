use std::path::PathBuf;

/// Validates a vector of paths.
///
/// Returns error on:
///
///     - Empty input vector
///     - Non-existing path in input vector
pub fn validate_paths(sources: Vec<PathBuf>) -> Result<(), String> {
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

    Ok(())
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jul 02 2021, 00:06 [CST]
