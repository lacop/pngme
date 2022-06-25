// TODO: Remove eventually, but avoids warning spam for now.
#![allow(dead_code)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

use anyhow::Result;
use clap::Parser;

use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;

fn get_file_byte(path: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    let f = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_file_bytes(path: &str, bytes: &[u8]) -> Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    f.write_all(bytes)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    println!("{:?}", cli); // TODO

    match &cli.command {
        args::Commands::Print { file } => {
            let file_bytes = get_file_byte(file)?;
            let png = Png::try_from(&file_bytes[..])?;
            for chunk in png.chunks() {
                if let Ok(s) = chunk.data_as_string() {
                    println!("{} => {}", chunk.chunk_type(), s);
                } else {
                    println!("{} (not text)", chunk.chunk_type());
                }
            }
        }
        args::Commands::Encode {
            file,
            chunk_type,
            message,
            output_file,
        } => {
            let file_bytes = get_file_byte(file)?;
            let mut png = Png::try_from(&file_bytes[..])?;

            if let Ok(chunk) = png.remove_chunk(chunk_type) {
                eprintln!(
                    "Warning: Removed existing chunk with contents: {}",
                    chunk
                        .data_as_string()
                        .ok()
                        .as_deref()
                        .unwrap_or("(not text)")
                );
            }

            let data: Vec<u8> = message.as_bytes().into();
            let chunk_type: [u8; 4] = chunk_type.as_bytes().try_into()?;
            let chunk_type = ChunkType::try_from(chunk_type)?;
            let chunk = Chunk::new(chunk_type, data);

            png.append_chunk(chunk);

            write_file_bytes(output_file.as_ref().unwrap_or(file), &png.as_bytes())?;
        }
        args::Commands::Decode { file, chunk_type } => {
            let file_bytes = get_file_byte(file)?;
            let png = Png::try_from(&file_bytes[..])?;

            if let Some(chunk) = png.chunk_by_type(chunk_type) {
                if let Ok(s) = chunk.data_as_string() {
                    println!("{} => {}", chunk_type, s);
                } else {
                    println!("{} => (not text)", chunk_type);
                }
            } else {
                println!("Chunk not found");
            }
        }
        args::Commands::Remove { file, chunk_type } => {
            let file_bytes = get_file_byte(file)?;
            let mut png = Png::try_from(&file_bytes[..])?;
            println!("BEFORE {} {}", png.chunks().len(), png.as_bytes().len());
            if let Ok(chunk) = png.remove_chunk(chunk_type) {
                println!("AFTER {} {}", png.chunks().len(), png.as_bytes().len());
                println!(
                    "Removed contents {}",
                    chunk
                        .data_as_string()
                        .ok()
                        .as_deref()
                        .unwrap_or("(not text)")
                );
                write_file_bytes(file, &png.as_bytes())?;
            } else {
                eprintln!("No such chunk");
            }
        }
    }

    Ok(())
}
