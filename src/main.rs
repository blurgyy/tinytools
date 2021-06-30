mod modules;

use std::path::PathBuf;

use structopt::StructOpt;

use modules::{bak::bak, gr::gr};

#[derive(StructOpt)]
#[structopt(name = "tinytools")]
enum TT {
    #[structopt(name = "bak")]
    /// Append a tilde (~) to the names of given files/directories.
    Bak {
        #[structopt(
            help = "The file(s)/director(y/ies) to append tilde.",
            parse(from_os_str)
        )]
        sources: Vec<PathBuf>,
        #[structopt(long = "--quiet", short = "-q", help = "Be quiet.")]
        quiet: bool,
    },
    #[structopt(name = "gr")]
    /// Get the nearest git root above current working directory (if it exists).
    Gr {},
}

fn main() -> Result<(), String> {
    match TT::from_args() {
        TT::Bak { mut sources, quiet } => {
            bak(&mut sources, quiet)?;
        }
        TT::Gr {} => {
            gr()?;
        }
    }
    Ok(())
}
// Author: Blurgy <gy@blurgy.xyz>
// Date:   Jun 30 2021, 12:39 [CST]
