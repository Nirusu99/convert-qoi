use std::path::PathBuf;

use clap::Parser;
use qoi::encode_to_vec;

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

    let img = image::io::Reader::open(args.source)?.decode()?;
    let width = img.width();
    let height = img.height();
    let encoded_qoi = encode_to_vec(&img.into_bytes(), width, height)?;
    std::fs::write(args.destination, encoded_qoi)?;
    Ok(())
}
