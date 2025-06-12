mod cli;
mod media;
mod archive;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Image { input, output } => {
            media::convert_image(&input, &output).expect("image conversion failed");
        }
        Commands::Archive { archive, output } => {
            archive::extract_archive(&archive, &output).expect("archive extraction failed");
        }
        Commands::Media { input, output } => {
            media::convert_media(&input, &output).expect("media conversion failed");
        }
    }
}
