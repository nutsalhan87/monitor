use serde::{Deserialize, Deserializer};

use super::Event;

fn perf_value<'de, D>(de: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(de).unwrap();
    match s.parse::<f32>() {
        Ok(value) => Ok(Some(value as u64)),
        Err(_) => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct PerfStat {
    interval: f32,
    #[serde(rename(deserialize = "counter-value"), deserialize_with = "perf_value")]
    counter_value: Option<u64>,
    unit: String,
    event: String,
}

impl TryInto<Event> for PerfStat {
    type Error = ();

    fn try_into(self) -> Result<Event, Self::Error> {
        if let Some(value) = self.counter_value {
            Ok(Event {
                timestamp_millis: (self.interval * 1000f32) as u32,
                description: self.event,
                value,
                unit: self.unit,
            })
        } else {
            Err(())
        }
    }
}

impl PerfStat {
    pub fn parse_events(output: &str) -> Vec<Event> {
        output
            .lines()
            .into_iter()
            .filter_map(|line| {
                serde_json::de::from_str::<PerfStat>(line).map_or(None, |v| v.try_into().ok())
            })
            .collect()
    }
}
