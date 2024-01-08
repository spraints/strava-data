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
        let record = record?;
        res.push(
            (&record)
                .try_into()
                .or_else(|e| Err(anyhow::anyhow!("{e} while parsing {}", record.as_slice())))?,
        );
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
    // todo: figure out which of these fields I'm interested in:
    //   Activity ID,Activity Date,Activity Name,Activity Type,Activity Description,Elapsed Time,Distance,Max Heart Rate,Relative Effort,Commute,Activity Private Note,Activity Gear,Filename,Athlete Weight,Bike Weight,Elapsed Time,Moving Time,Distance,Max Speed,Average Speed,Elevation Gain,Elevation Loss,Elevation Low,Elevation High,Max Grade,Average Grade,Average Positive Grade,Average Negative Grade,Max Cadence,Average Cadence,Max Heart Rate,Average Heart Rate,Max Watts,Average Watts,Calories,Max Temperature,Average Temperature,Relative Effort,Total Work,Number of Runs,Uphill Time,Downhill Time,Other Time,Perceived Exertion,Type,Start Time,Weighted Average Power,Power Count,Prefer Perceived Exertion,Perceived Relative Effort,Commute,Total Weight Lifted,From Upload,Grade Adjusted Distance,Weather Observation Time,Weather Condition,Weather Temperature,Apparent Temperature,Dewpoint,Humidity,Weather Pressure,Wind Speed,Wind Gust,Wind Bearing,Precipitation Intensity,Sunrise Time,Sunset Time,Moon Phase,Bike,Gear,Precipitation Probability,Precipitation Type,Cloud Cover,Weather Visibility,UV Index,Weather Ozone,Jump Count,Total Grit,Average Flow,Flagged,Average Elapsed Speed,Dirt Distance,Newly Explored Distance,Newly Explored Dirt Distance,Activity Count,Total Steps,Media
    // - My initial point of curiosity was date,activity_type,max_heart_rate so that I could see how
    //   heart rate varied per activity. If avg heart rate is available, that would be nice to see,
    //   too.
    // - I think I'd also be interested in plotting distance and time for run and/or bike.
}

impl TryFrom<&StringRecord> for ActivitySummary {
    type Error = anyhow::Error;

    fn try_from(value: &StringRecord) -> Result<Self, Self::Error> {
        fn f(v: &StringRecord, i: usize) -> anyhow::Result<&str> {
            v.get(i).ok_or_else(|| {
                anyhow::anyhow!("tried to get field {i} from row with {} fields", v.len())
            })
        }
        Ok(Self {
            id: f(&value, 0)?.parse()?,
            // todo: chrono seems to require either space-padded or 0-padded day number and hour.
            // >:(
            date: NaiveDateTime::parse_from_str(f(&value, 1)?, "%b %-d, %Y, %-H:%M:%S %p")?
                .and_utc(),
        })
    }
}
