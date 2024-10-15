#![feature(generic_const_exprs)]

pub mod hash;
pub mod sha256;

use std::{fs::File, io::{stdin, Read}, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use hash::{HashAlgorithm, Update};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value_t=0x1000)]
    chunk_size: usize,

    input: Option<PathBuf>
}

fn open_or_stdin(path: Option<PathBuf>) -> Result<Box<dyn Read>> {
    Ok(if let Some(path) = path {
        Box::new(File::open(&path).context(format!("Could not open {:?}", &path))?)
    } else {
        Box::new(stdin().lock())
    })
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let mut input = open_or_stdin(args.input)?;

    let mut hasher = sha256::SHA256::default();
    loop {
        let mut buffer = vec![0u8; args.chunk_size];
        let size = input.read(&mut buffer)?;
        if size == 0 {
            break
        }
        hasher.update(&buffer[..size]);
    }

    let hash = hasher.finalize();

    println!("{}", hash.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join(""));

    Ok(())
}
