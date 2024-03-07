use std::{
    fs::read_to_string,
    sync::{mpsc::Sender, RwLock, Arc},
    thread,
    time::{Duration, Instant},
};

use super::{Event, Prof};

pub struct ProcMem {}

impl Prof for ProcMem {
    fn profiler(
        &self,
        freq_millis: u32,
        _pid: u32,
        sender: Sender<Event>,
        is_program_end: Arc<RwLock<bool>>,
    ) {
        let start_time = Instant::now();
        let total = mem_total();
        while !*is_program_end.read().unwrap() {
            thread::sleep(Duration::from_millis(freq_millis as u64));
            let timestamp_millis = Instant::now().duration_since(start_time).as_millis() as u32;
            let value = mem_avaliable();
            sender
                .send(Event {
                    timestamp_millis,
                    description: "Memory occupied".to_string(),
                    value: total - value,
                    unit: "kB".to_string(),
                })
                .unwrap();
        }
    }
}

fn mem_total() -> u64 {
    let meminfo = read_to_string("/proc/meminfo").unwrap();
    let mut total_line = meminfo.lines().next().unwrap().split_whitespace().skip(1);
    total_line.next().unwrap().parse::<u64>().unwrap()
}

fn mem_avaliable() -> u64 {
    let meminfo = read_to_string("/proc/meminfo").unwrap();
    let mut avaliable_line = meminfo
        .lines()
        .skip(2)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1);
    avaliable_line.next().unwrap().parse::<u64>().unwrap()
}
