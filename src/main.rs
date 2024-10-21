#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

pub mod hash;
pub mod cipher;
pub mod sha256;
pub mod aes;

use std::{
    fs::File, io::{Read, Write}, path::PathBuf
};

use hex::{FromHex, ToHex};
use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use hash::DynHashAlgorithm;
use cipher::BlockCipher;

pub fn from_hex(s: &str) -> Result<Vec<u8>> {
    Ok(<Vec<u8>>::from_hex(s)?)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hex(Vec<u8>);

impl std::ops::Deref for Hex {
    type Target=Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Hex {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(<Vec<u8>>::from_hex(s)?))
    }
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum BlockAlgorithm {
    AES128,
    AES192,
    AES256,
}

#[derive(Debug, Clone, Parser)]
enum Args {
    Hash {
        input: Option<PathBuf>
    },
    BlockEncrypt {
        algo: BlockAlgorithm,
        key: Hex,
        input: Option<PathBuf>,
        output: Option<PathBuf>,
    }
}

fn open_or_stdin(path: Option<PathBuf>) -> Result<Box<dyn Read>> {
    Ok(if let Some(path) = path {
        Box::new(File::open(&path).context(format!("Could not open {:?}", &path))?)
    } else {
        Box::new(std::io::stdin().lock())
    })
}

fn open_or_stdout(path: Option<PathBuf>) -> Result<Box<dyn Write>> {
    Ok(if let Some(path) = path {
        Box::new(File::open(&path).context(format!("Could not open {:?}", &path))?)
    } else {
        Box::new(std::io::stdout().lock())
    })
}

pub fn ecb_block_encrypt<Cipher: BlockCipher>(key: [u8; Cipher::KEY_SIZE], mut reader: impl Read, mut writer: impl Write) -> Result<()>
        where [(); Cipher::BLOCK_SIZE]: {
    let cipher = Cipher::new(key);
    loop {
        let mut block = [0u8; Cipher::BLOCK_SIZE];
        reader.read_exact(&mut block)?; // fix last block
        let block = cipher.encrypt(block);
        writer.write(&block)?;
        writer.flush();
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    match args {
        Args::Hash{input}=> {
            let mut input = open_or_stdin(input)?;

            let mut hasher = sha256::SHA256::default();

            std::io::copy(&mut input, &mut hasher)?;

            let hash = hasher.finalize();

            println!("{}", hash.encode_hex::<String>());
        },
        Args::BlockEncrypt {algo, key, input, output} => {
            let input = open_or_stdin(input)?;
            let output = open_or_stdout(output)?;
            match algo {
                BlockAlgorithm::AES128 => {
                    ecb_block_encrypt::<aes::AES128>(key.0.try_into().unwrap(), input, output)?;
                },
                BlockAlgorithm::AES192 => {
                    ecb_block_encrypt::<aes::AES192>(key.0.try_into().unwrap(), input, output)?;
                },
                BlockAlgorithm::AES256 => {
                    ecb_block_encrypt::<aes::AES256>(key.0.try_into().unwrap(), input, output)?;
                },
            }
        },
    }

    Ok(())

}
