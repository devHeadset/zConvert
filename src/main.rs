mod cli;
mod media;
mod archive;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Convert { input, output } => {
            media::convert_media(&input, &output).expect("conversion failed");
        }
        Commands::Extract { archive_path, out_dir } => {
            archive::extract_archive(&archive_path, &out_dir).expect("extraction failed");
        }
    }
}
