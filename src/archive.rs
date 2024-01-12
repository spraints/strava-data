use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Default)]
pub struct Archive {
    pub activities: Vec<ActivitySummary>,
}

#[derive(Deserialize, Debug)]
pub struct ActivitySummary {
    #[serde(rename = "Activity ID")]
    pub id: u128,
    #[serde(rename = "Activity Date", deserialize_with = "deserialize_date")]
    pub date: DateTime<Utc>,
    #[serde(rename = "Activity Name")]
    pub name: String,
    #[serde(rename = "Activity Type")]
    pub act_type: ActivityType,
    #[serde(rename = "Elapsed Time")]
    pub elapsed_seconds: u32,
    #[serde(rename = "Distance")]
    pub distance: f32,
    #[serde(rename = "Max Heart Rate")]
    pub max_heart_rate: Option<f32>,
    #[serde(rename = "Filename")]
    pub filename: String,
    #[serde(rename = "Elapsed Time (2)")]
    pub elapsed_seconds2: Option<f32>,
    #[serde(rename = "Moving Time")]
    pub moving_seconds: f32,
    #[serde(rename = "Distance (2)")]
    pub distance2: f32,
    #[serde(rename = "Max Speed")]
    pub max_speed: f32,
    #[serde(rename = "Average Speed")]
    pub avg_speed: Option<f32>,
    #[serde(rename = "Elevation Low")]
    pub elevation_low: f32,
    #[serde(rename = "Elevation High")]
    pub elevation_high: f32,
    // - My initial point of curiosity was date,activity_type,max_heart_rate so that I could see how
    //   heart rate varied per activity. If avg heart rate is available, that would be nice to see,
    //   too.
    // - I think I'd also be interested in plotting distance and time for run and/or bike.
}

#[derive(Deserialize, Debug)]
pub enum ActivityType {
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
