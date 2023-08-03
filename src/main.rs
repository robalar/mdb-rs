use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{command, Parser};

use mdb_rs::parser::{parse_access_file, Page};
use rc4::{consts::U128, Key, KeyInit, Rc4, StreamCipher};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bytes = fs::read(args.file)?;

    let database = parse_access_file(bytes.as_slice());

    if let Page::DatabaseDefinition(x) = &database.pages[0].val {
        dbg!(&x.key);
        let key = Key::<U128>::from_slice(&x.rc4_key);
        let mut rc4 = Rc4::<_>::new(key);
        rc4.apply_keystream(&mut x.key.to_le_bytes());

        dbg!(x.key);
    }

    Ok(())
}
