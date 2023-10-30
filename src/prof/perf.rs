use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

impl Into<Event> for PerfStat {
    fn into(self) -> Event {
        Event {
            timestamp_millis: (self.interval * 1000f32) as u32,
            description: self.event,
            value: self.counter_value,
            unit: self.unit,
        }
    }
}

impl PerfStat {
    pub fn new(pipe: File) -> Vec<Event> {
        let pipe_reader = BufReader::new(pipe);
        let pipe_lines = pipe_reader.lines();
        let mut stats: Vec<Event> = Vec::new();
        for line in pipe_lines.map(|v| v.unwrap()).skip(2) {
            stats.push(serde_json::de::from_str::<PerfStat>(&line).unwrap().into());
        }
        stats
    }
}
