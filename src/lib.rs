use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn buf_reader_from_arg() -> Result<BufReader<File>, ()> {
    // TODO: Fix result, add error handling
    let args: Vec<String> = std::env::args().collect();
    let bin_path = Path::new(&args[0]);

    if args.len() != 2 {
        println!("Usage: {} <path>", bin_path.file_name().unwrap().to_str().unwrap());
    }
    Ok(BufReader::new(File::open(&args[1]).unwrap()))
}

pub fn parse_lines<T>(reader: BufReader<File>) -> impl Iterator<Item = T> where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    reader.lines().map(|l| l.unwrap().parse::<T>().unwrap())
}
