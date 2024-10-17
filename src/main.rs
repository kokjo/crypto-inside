#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

pub mod hash;
pub mod cipher;
pub mod sha256;
pub mod aes;

use std::{
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;
use hash::DynHashAlgorithm;

#[derive(Debug, Parser)]
struct Args {
    input: Option<PathBuf>,
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

    std::io::copy(&mut input, &mut hasher)?;

    let hash = hasher.finalize();

    println!("{}", hash.iter().map(|byte| format!("{:02x}", byte)).collect::<Vec<_>>().join(""));

    Ok(())
}
