use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::archive::ActivityType;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Verify that the archive can be parsed.
    Verify(VerifyArgs),

    /// Show a metric for an activity over time.
    Trend(TrendArgs),
}

#[derive(Args)]
pub struct VerifyArgs {
    /// The directory that was extracted from the archive downloaded from
    /// https://www.strava.com/athlete/delete_your_account.
    pub dir: String,
}

#[derive(Args)]
pub struct TrendArgs {
    /// The directory that was extracted from the archive downloaded from
    /// https://www.strava.com/athlete/delete_your_account.
    #[arg(short, long)]
    pub dir: String,

    /// The activity to inspect.
    #[arg(short, long)]
    pub activity: ActivityType,

    /// The metric to inspect.
    #[arg(short, long)]
    pub metric: MetricType,
}

#[derive(Clone, ValueEnum)]
pub enum MetricType {
    Duration,
    Distance,
    HeartRate,
    Elevation,
}
