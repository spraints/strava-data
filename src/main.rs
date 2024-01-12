mod archive;
mod args;

use archive::{ActivitySummary, Archive};
use args::{Cli, Command};
use chrono::{DateTime, Local};
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
        Command::Trend(args) => {
            if let Err(e) = show_trend(args) {
                println!("error: {}", e);
            }
        }
    };
}

fn show_trend(args: args::TrendArgs) -> anyhow::Result<()> {
    let args::TrendArgs {
        dir,
        activity: act_type,
        metric,
    } = args;
    let archive = parse(dir)?;
    let (fields, reporter) = init_reporter(metric);

    print!("Date            ");
    for f in fields {
        print!(" {f:>10}");
    }
    println!();

    for activity in archive.activities {
        if activity.act_type == act_type {
            if let Some(fields) = (reporter.fields)(&activity) {
                let local: DateTime<Local> = DateTime::from(activity.date);
                print!("{}", local.format("%Y-%m-%d %H:%M"));
                for f in fields {
                    print!(" {f:>10}");
                }
                println!();
            }
        }
    }
    Ok(())
}

fn init_reporter(metric: args::MetricType) -> (Vec<&'static str>, Reporter) {
    match metric {
        args::MetricType::Duration => (
            vec!["moving", "elapsed"],
            Reporter {
                fields: duration_trend_row,
            },
        ),
        args::MetricType::Distance => (
            vec!["distance", "duration", "avg pace", "fastest"],
            Reporter {
                fields: distance_trend_row,
            },
        ),
        args::MetricType::HeartRate => (
            vec!["avg hr", "max hr", "avg pace", "max pace"],
            Reporter {
                fields: heart_rate_trend_row,
            },
        ),
        args::MetricType::Elevation => (
            vec!["low", "high"],
            Reporter {
                fields: elevation_trend_row,
            },
        ),
    }
}

type Row = Option<Vec<Cow<'static, str>>>;

fn duration_trend_row(act: &ActivitySummary) -> Row {
    Some(vec![
        mmss(act.moving_seconds as u32).into(),
        mmss(act.elapsed_seconds).into(),
    ])
}

fn mmss(sec: u32) -> String {
    let min = sec / 60;
    let sec = sec % 60;
    format!("{min}:{sec:02}")
}

fn distance_trend_row(act: &ActivitySummary) -> Row {
    Some(vec![
        format!("{:0.02} km", act.distance).into(),
        mmss(act.moving_seconds as u32).into(),
        act.avg_speed
            .map(km_pace)
            .map(Into::into)
            .unwrap_or("".into()),
        km_pace(act.max_speed).into(),
    ])
}

fn km_pace(meters_per_second: f32) -> String {
    let seconds_per_km = 1000.0 / meters_per_second;
    format!("{}/km", mmss(seconds_per_km as u32))
}

fn heart_rate_trend_row(act: &ActivitySummary) -> Row {
    match (act.avg_heart_rate, act.max_heart_rate) {
        (Some(ahr), Some(max)) => Some(vec![
            format!("{ahr:.0}").into(),
            format!("{max:.0}").into(),
            act.avg_speed
                .map(km_pace)
                .map(Into::into)
                .unwrap_or("".into()),
        ]),
        _ => None,
    }
}

fn elevation_trend_row(act: &ActivitySummary) -> Row {
    Some(vec![
        format!("{:.0} m", act.elevation_low).into(),
        format!("{:.0} m", act.elevation_high).into(),
    ])
}

struct Reporter {
    fields: fn(&ActivitySummary) -> Row,
}

fn parse<P: AsRef<Path>>(dir: P) -> anyhow::Result<Archive> {
    let mut res = Archive::default();
    for e in dir.as_ref().read_dir()? {
        parse_dir_entry(&mut res, e?)?;
    }
    Ok(res)
}

fn parse_dir_entry(archive: &mut Archive, e: DirEntry) -> anyhow::Result<()> {
    if let Some("activities.csv") = e.file_name().to_str() {
        archive.activities = parse_activities_csv(e.path())?;
    }
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
