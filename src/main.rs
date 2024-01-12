mod archive;
mod args;

use archive::Archive;
use args::{Cli, Command};
use clap::Parser;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::fs::DirEntry;
use std::path::Path;

fn main() {
    match Cli::parse().command {
        Command::Verify(args) => match parse(&args.dir) {
            Ok(archive) => {
                println!("{}: OK!", args.dir);
                println!("activities:");
                println!("  {:?}", archive.activities[0]);
                println!("  ...");
                println!("  {:?}", archive.activities[archive.activities.len() - 1]);
            }
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
        Some("activities.csv") => archive.activities = parse_activities_csv(e.path())?,
        _ => (),
    };
    Ok(())
}

fn parse_activities_csv<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> anyhow::Result<Vec<T>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let fixed_headers = fix_headers(rdr.headers()?);
    rdr.set_headers(fixed_headers);
    let mut res = Vec::new();
    for record in rdr.deserialize() {
        res.push(record?);
    }
    Ok(res)
}

fn fix_headers(header: &csv::StringRecord) -> csv::StringRecord {
    let mut headers: Vec<Cow<str>> = Vec::new();
    for field in header {
        if headers.contains(&Cow::Borrowed(field)) {
            for suffix in 2.. {
                let new_field = Cow::Owned(format!("{field} ({suffix})"));
                if !headers.contains(&new_field) {
                    headers.push(new_field);
                    break;
                }
            }
        } else {
            headers.push(Cow::Borrowed(field));
        }
    }
    headers.into()
}
