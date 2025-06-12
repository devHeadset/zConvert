use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zConvert")]
#[command(about = "convert media or extract archives", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// convert media using ffmpeg
    Convert {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    /// extract an archive
    Extract {
        #[arg(short, long)]
        archive_path: String,

        #[arg(short, long, default_value = "./output")]
        out_dir: String,
    },
}
