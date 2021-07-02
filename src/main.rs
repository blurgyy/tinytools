mod modules;

use std::path::PathBuf;
use structopt::StructOpt;

use modules::{bak::bak, debak::debak, gr::gr};

#[derive(StructOpt)]
#[structopt(
    name = "tinytools",
    global_settings(&[structopt::clap::AppSettings::ColoredHelp])
)]
enum TT {
    #[structopt(name = "bak")]
    /// Append a tilde (~) to the names of given files/directories.
    Bak {
        #[structopt(
            help = "The file/directory to be renamed",
            parse(from_os_str)
        )]
        source: PathBuf,
        #[structopt(
            help = "More files/directories to be renamed",
            parse(from_os_str)
        )]
        more_sources: Vec<PathBuf>,
        #[structopt(long = "--quiet", short = "-q", help = "Be quiet")]
        quiet: bool,
    },
    #[structopt(name = "debak")]
    /// Pop a tilde (~) from the names of given files/directories
    Debak {
        #[structopt(
            help = "The file/directory to be renamed",
            parse(from_os_str)
        )]
        source: PathBuf,
        #[structopt(
            help = "More files/directories to be renamed",
            parse(from_os_str)
        )]
        more_sources: Vec<PathBuf>,
        #[structopt(long = "--quiet", short = "-q", help = "Be quiet")]
        quiet: bool,
    },
    #[structopt(name = "gr")]
    /// Get the nearest git root above current working directory (if it exists).
    Gr {},
}

fn main() -> Result<(), String> {
    match TT::from_args() {
        TT::Bak {
            source,
            more_sources,
            quiet,
        } => {
            bak(source, more_sources, quiet)?;
        }
        TT::Debak {
            source,
            more_sources,
            quiet,
        } => {
            debak(source, more_sources, quiet)?;
        }
        TT::Gr {} => {
            gr()?;
        }
    }
    Ok(())
}
// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jun 30 2021, 12:39 [CST]
