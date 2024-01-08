mod args;

use args::{Cli, Command};
use chrono::{DateTime, NaiveDateTime, Utc};
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
    date: DateTime<Utc>,
    name: String,
    act_type: ActivityType,
    elapsed_seconds: u32,
    distance: f32,
    max_heart_rate: Option<f32>,
    filename: String,
    elapsed_seconds2: Option<f32>,
    moving_seconds: Option<f32>,
    distance2: f32,
    max_speed: Option<f32>,
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
        fn f<'b>(v: &'b StringRecord, i: usize, label: &str) -> anyhow::Result<&'b str> {
            v.get(i).ok_or_else(|| {
                anyhow::anyhow!(
                    "tried to get {label} field {i} from row with {} fields",
                    v.len()
                )
            })
        }
        Ok(Self {
            // todo - figure out which of these fields are worth keeping.
            // todo - use serde?
            id: f(&value, 0, "Activity ID")?.parse()?,
            date: parse_date(f(&value, 1, "Activity Date")?)?,
            name: f(&value, 2, "Activity Name")?.to_string(),
            act_type: f(&value, 3, "Activity Type")?.try_into()?,
            elapsed_seconds: f(&value, 5, "Elapsed Time")?.parse()?,
            distance: f(&value, 6, "Distance")?.parse()?,
            max_heart_rate: match f(&value, 7, "Max Heart Rate")? {
                "" => None,
                s => Some(s.parse()?),
            },
            filename: f(&value, 12, "Filename")?.to_string(),
            elapsed_seconds2: match f(&value, 15, "Elapsed Time")? {
                "" => None,
                s => Some(s.parse()?),
            },
            moving_seconds: match f(&value, 16, "Moving Time")? {
                "" => None,
                s => Some(s.parse()?),
            },
            distance2: f(&value, 17, "Distance")?.parse()?,
            max_speed: match f(&value, 18, "Max Speed")? {
                "" => None,
                s => Some(s.parse()?),
            },
            // todo - parse some more
        })
    }
}

enum ActivityType {
    AlpineSki,
    Hike,
    IceSkate,
    Ride,
    Run,
    Walk,
    WeightTraining,
    Workout,
    Yoga,
}

impl TryFrom<&str> for ActivityType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // todo - use serde for this??
        let res = match value {
            "Alpine Ski" => ActivityType::AlpineSki,
            "Hike" => ActivityType::Hike,
            "Ice Skate" => ActivityType::IceSkate,
            "Ride" => ActivityType::Ride,
            "Run" => ActivityType::Run,
            "Walk" => ActivityType::Walk,
            "Weight Training" => ActivityType::WeightTraining,
            "Workout" => ActivityType::Workout,
            "Yoga" => ActivityType::Yoga,
            _ => anyhow::bail!("unrecognized activity type {value:?}"),
        };
        Ok(res)
    }
}

fn parse_date(date_str: &str) -> anyhow::Result<DateTime<Utc>> {
    Ok(
        NaiveDateTime::parse_from_str(date_str, "%b %-d, %Y, %-I:%M:%S %p")
            .or_else(|e| {
                Err(anyhow::anyhow!(
                    "could not parse date string {date_str:?}: {e}"
                ))
            })?
            .and_utc(),
    )
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    #[test]
    fn parse_time() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2014, 12, 26)
                .unwrap()
                .and_hms_opt(20, 2, 53)
                .unwrap()
                .and_utc(),
            super::parse_date("Dec 26, 2014, 8:02:53 PM").unwrap()
        );

        assert_eq!(
            NaiveDate::from_ymd_opt(2024, 1, 7)
                .unwrap()
                .and_hms_opt(1, 34, 17)
                .unwrap()
                .and_utc(),
            super::parse_date("Jan 7, 2024, 1:34:17 AM").unwrap()
        );
    }
}
