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
    /// convert images (jpg, png, avif, etc)
    Image {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    },

    /// convert or extract archives
    Archive {
        #[arg(short = 'a', long)]
        archive: String,
        #[arg(short = 'o', long, default_value = "./output")]
        output: String,
    },

    /// convert media files (video/audio)
    Media {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: String,
    },
}

