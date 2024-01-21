mod config;

use eyre::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    if env::args().len() != 2 {
        eprintln!(
            "Usage: {} <path-to-config-file>",
            env::args().next().unwrap()
        );
        std::process::exit(1);
    }
    let path_to_config = PathBuf::from(env::args().nth(1).unwrap());

    let config = config::Config::try_from(path_to_config)?;

    Ok(())
}
