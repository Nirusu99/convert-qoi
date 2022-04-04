use std::path::PathBuf;

use clap::Parser;
use image::ColorType;
use qoi::{decode_to_vec, encode_to_vec, Channels};

use consts::*;

mod consts;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    source: PathBuf,
    #[clap(short, long, parse(from_os_str))]
    destination: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_ext = match args.source.extension() {
        Some(ext) => ext.to_str(),
        None => {
            println!("Error: {}", INVALID_SOURCE_FILE_EXT);
            return Ok(());
        }
    };

    let dest_ext = match args.destination.extension() {
        Some(ext) => ext.to_str(),
        None => {
            println!("Error: {}", INVALID_DEST_FILE_EXT);
            return Ok(());
        }
    };

    match (source_ext, dest_ext) {
        (Some("qoi"), Some("qoi")) => {
            std::fs::copy(&args.source, &args.destination)?;
            return Ok(());
        }
        (Some(source_ext), Some(dest_ext)) => {
            let src_is_image = is_image_ext(source_ext);
            if dest_ext == "qoi" && src_is_image {
                let img = image::io::Reader::open(&args.source)?.decode()?;
                let width = img.width();
                let height = img.height();
                let encoded_bytes = encode_to_vec(&img.into_bytes(), width, height)?;
                std::fs::write(args.destination, encoded_bytes)?;
            } else if source_ext == "qoi" && is_image_ext(dest_ext) {
                let read = std::fs::read(&args.source)?;
                let decoded_qoi = decode_to_vec(&read)?;
                let color = match decoded_qoi.0.channels {
                    Channels::Rgb => ColorType::Rgb8,
                    Channels::Rgba => ColorType::Rgba8,
                };
                image::save_buffer(
                    args.destination,
                    &decoded_qoi.1,
                    decoded_qoi.0.width,
                    decoded_qoi.0.height,
                    color,
                )?;
            } else {
                println!(
                    "Error: {} = {} -> {}",
                    INVALID_FILE_EXT, source_ext, dest_ext
                );
            }
            return Ok(());
        }
        (source_ext, dest_ext) => {
            println!(
                "Error: {} = {:?} -> {:?}",
                INVALID_FILE_EXT, source_ext, dest_ext
            );
            return Ok(());
        }
    }
}

fn is_image_ext(ext: &str) -> bool {
    IMAGE_FORMATS.iter().any(|s| *s == ext)
}
