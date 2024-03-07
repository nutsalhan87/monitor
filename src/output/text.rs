use crate::prof::Event;

use super::Outputter;

pub struct TextOutput;

impl Outputter for TextOutput {
    fn output(&self, events: &[Event]) {
        for event in events {
            println!(
                "{:.2}   {}: {} {}",
                event.timestamp_millis, event.description, event.value, event.unit
            )
        }
    }
}
