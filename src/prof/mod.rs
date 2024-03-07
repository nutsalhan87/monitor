mod perf;
mod proccpu;
mod procio;
mod procmaps;
mod procmem;
mod procnet;

pub use perf::PerfStat;
pub use proccpu::ProcCpu;
pub use procio::ProcIo;
pub use procmaps::ProcMaps;
pub use procmem::ProcMem;
pub use procnet::ProcNet;

use std::sync::{mpsc::Sender, RwLock, Arc};

#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp_millis: u32,
    pub description: String,
    pub value: u64,
    pub unit: String
}

pub trait Prof: Sync + Send {
    fn profiler(&self, freq_millis: u32, pid: u32, sender: Sender<Event>, is_program_end: Arc<RwLock<bool>>);
}

pub fn profs(events: Vec<String>) -> Vec<Box<dyn Prof>> {
    let mut p: Vec<Box<dyn Prof>> = Vec::new();
    for event in events {
        match event.as_str() {
            "maps" => p.push(Box::new(ProcMaps {})),
            "mem" => p.push(Box::new(ProcMem {})),
            "io" => p.push(Box::new(ProcIo {})),
            "cpu" => p.push(Box::new(ProcCpu {})),
            "net" => p.push(Box::new(ProcNet {})),
            _ => (),
        }
    }
    p
}
