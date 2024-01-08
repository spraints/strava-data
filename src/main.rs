mod args;

use args::{Cli, Command};
use clap::Parser;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fs::{DirEntry, File};
use std::path::Path;

fn main() {
    match Cli::parse().command {
        Command::Verify(args) => match parse(&args.dir) {
            Ok(_) => println!("{}: OK!", args.dir),
            Err(e) => println!("{}: error: {}", args.dir, e),
        },
    };
}

fn parse<P: AsRef<Path>>(dir: P) -> anyhow::Result<Archive> {
    let mut res = Archive::default();
    for e in dir.as_ref().read_dir()? {
        parse_dir_entry(&mut res, e?)?;
    }
    Ok(res)
}

fn parse_dir_entry(archive: &mut Archive, e: DirEntry) -> anyhow::Result<()> {
    match e.file_name().to_str() {
        Some("flags.csv") => archive.flags = parse_file(e.path())?,
        Some("media.csv") => archive.media = parse_file(e.path())?,
        _ => anyhow::bail!("{}: unrecognized file name", e.path().to_string_lossy()),
    };
    Ok(())
}

fn parse_file<P: AsRef<Path>, T: DeserializeOwned>(filename: P) -> anyhow::Result<Vec<T>> {
    let r = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(r);
    let mut res = Vec::new();
    for x in rdr.deserialize() {
        let record: T = x?;
        res.push(record);
    }
    Ok(res)
}

#[derive(Default)]
struct Archive {
    flags: Vec<Flag>,
    media: Vec<Media>,
}

#[derive(Deserialize)]
struct Flag {
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Flagged Type")]
    flagged_type: String,
    #[serde(rename = "Flagged ID")]
    flagged_id: String,
    #[serde(rename = "Comments")]
    comments: String,
    #[serde(rename = "Created At")]
    created_at: String,
}

#[derive(Deserialize)]
struct Media {
    #[serde(rename = "Media Filename")]
    filename: String,
    #[serde(rename = "Media Caption")]
    caption: String,
}
