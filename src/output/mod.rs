mod plain;
mod plot;

pub use plain::PlainOutput;
pub use plot::PlotOutput;

use crate::prof::Event;

pub trait Outputter {
    fn output(&mut self, events: &[Event]);
}
