mod args;

use args::{Cli, Command};
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::Parser;
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
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

#[derive(Default)]
struct Archive {
    activities: Vec<ActivitySummary>,
}

#[derive(Deserialize)]
struct ActivitySummary {
    #[serde(rename = "Activity ID")]
    id: u128,
    #[serde(rename = "Activity Date", deserialize_with = "deserialize_date")]
    date: DateTime<Utc>,
    #[serde(rename = "Activity Name")]
    name: String,
    #[serde(rename = "Activity Type")]
    act_type: ActivityType,
    #[serde(rename = "Elapsed Time")]
    elapsed_seconds: u32,
    #[serde(rename = "Distance")]
    distance: f32,
    #[serde(rename = "Max Heart Rate")]
    max_heart_rate: Option<f32>,
    #[serde(rename = "Filename")]
    filename: String,
    #[serde(rename = "Elapsed Time (2)")]
    elapsed_seconds2: Option<f32>,
    #[serde(rename = "Moving Time")]
    moving_seconds: Option<f32>,
    #[serde(rename = "Distance (2)")]
    distance2: f32,
    #[serde(rename = "Max Speed")]
    max_speed: Option<f32>,
    #[serde(rename = "Average Speed")]
    avg_speed: Option<f32>,
    #[serde(rename = "Elevation Low")]
    elevation_low: Option<f32>,
    #[serde(rename = "Elevation High")]
    elevation_high: Option<f32>,
    // - My initial point of curiosity was date,activity_type,max_heart_rate so that I could see how
    //   heart rate varied per activity. If avg heart rate is available, that would be nice to see,
    //   too.
    // - I think I'd also be interested in plotting distance and time for run and/or bike.
}

#[derive(Deserialize)]
enum ActivityType {
    #[serde(rename = "Alpine Ski")]
    AlpineSki,
    Hike,
    #[serde(rename = "Ice Skate")]
    IceSkate,
    Ride,
    Run,
    Walk,
    #[serde(rename = "Weight Training")]
    WeightTraining,
    Workout,
    Yoga,
}

fn deserialize_date<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d)
        .and_then(|s| parse_date(&s).map_err(|e| serde::de::Error::custom(e.to_string())))
}

fn parse_date(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    NaiveDateTime::parse_from_str(&s, "%b %-d, %Y, %-I:%M:%S %p").map(|dt| dt.and_utc())
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
