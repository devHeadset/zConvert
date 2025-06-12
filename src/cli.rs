use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zconvert")]
#[command(about = "convert images and extract archives", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// convert image files (e.g., jpg to png)
    Image {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    /// extract archive files (zip, tar, etc)
    Archive {
        #[arg(short = 'a', long)]
        archive: String,

        #[arg(short = 'o', long, default_value = "./output")]
        output: String,
    },
}

