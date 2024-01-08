mod args;

use args::{Cli, Command};
use chrono::{NaiveDateTime, Utc};
use clap::Parser;
use csv::StringRecord;
use std::fs::DirEntry;
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
        Some("activities.csv") => archive.activities = parse_activities_csv(e.path())?,
        _ => (),
    };
    Ok(())
}

fn parse_activities_csv<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<ActivitySummary>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut res = Vec::new();
    for record in rdr.records().skip(1) {
        res.push(record?.try_into()?);
    }
    Ok(res)
}

#[derive(Default)]
struct Archive {
    activities: Vec<ActivitySummary>,
}

struct ActivitySummary {
    id: u128,
    date: chrono::DateTime<Utc>,
}

impl TryFrom<StringRecord> for ActivitySummary {
    type Error = anyhow::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        fn f(v: &StringRecord, i: usize) -> anyhow::Result<&str> {
            v.get(i).ok_or_else(|| {
                anyhow::anyhow!("tried to get field {i} from row with {} fields", v.len())
            })
        }
        Ok(Self {
            id: f(&value, 0)?.parse()?,
            date: NaiveDateTime::parse_from_str(f(&value, 1)?, "%b %d, %Y, %H:%M:%S %p")?.and_utc(),
        })
    }
}
