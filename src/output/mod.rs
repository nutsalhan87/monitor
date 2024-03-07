mod text;
mod plot;

pub use text::TextOutput;
pub use plot::PlotOutput;

use crate::prof::Event;

pub trait Outputter {
    fn output(&self, events: &[Event]);
}
