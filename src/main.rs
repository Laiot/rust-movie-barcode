use std::fs::File;
use std::io::{BufReader};
use mp4::{Result};
static MOVIE_PATH: &'static str = "/home/laiot/Documents/rust-movie-barcode/resources/rainingwhereioncewas.mp4";

fn main() -> Result<()>{
    let f = File::open(MOVIE_PATH).unwrap();
    let size = f.metadata()?.len();
    let reader = BufReader::new(f);
    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;
    println!("Hello, world!");
    println!("major brand: {}", mp4.ftyp.major_brand);
    println!("duration: {:?}", mp4.duration());

    Ok(())
}
