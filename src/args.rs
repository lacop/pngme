use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode {
        file: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    Decode {
        file: String,
        chunk_type: String,
    },
    Print {
        file: String,
    },
    Remove {
        file: String,
        chunk_type: String,
    },
}
