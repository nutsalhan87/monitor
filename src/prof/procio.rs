use std::{
    fs::{read_dir, read_to_string},
    sync::{mpsc::Sender, RwLock, Arc},
    thread,
    time::{Duration, Instant},
};

use super::{Event, Prof};

pub struct ProcIo {}

impl Prof for ProcIo {
    fn profiler(
        &self,
        freq_millis: u32,
        pid: u32,
        sender: Sender<Event>,
        is_program_end: Arc<RwLock<bool>>,
    ) {
        let mut read_bytes_prev = 0;
        let mut write_bytes_prev = 0;
        let start_time = Instant::now();
        while !*is_program_end.read().unwrap() {
            thread::sleep(Duration::from_millis(freq_millis.into()));
            let timestamp_millis = Instant::now().duration_since(start_time).as_millis() as u32;
            let (read_bytes, write_bytes) = rwbytes(pid);
            sender
                .send(Event {
                    timestamp_millis,
                    description: "io read".to_string(),
                    value: read_bytes - read_bytes_prev,
                    unit: "bytes".to_string(),
                })
                .unwrap();
            sender
                .send(Event {
                    timestamp_millis,
                    description: "io write".to_string(),
                    value: write_bytes - write_bytes_prev,
                    unit: "bytes".to_string(),
                })
                .unwrap();
            read_bytes_prev = read_bytes;
            write_bytes_prev = write_bytes;
        }
    }
}

fn rwbytes(pid: u32) -> (u64, u64) {
    let io = read_to_string(format!("/proc/{pid}/io"));
    if let Err(_) = io {
        return (0, 0);
    }
    let io = io.unwrap();
    let mut lines = io.lines().skip(4);
    let read_bytes: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let write_bytes: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let task = read_dir(format!("/proc/{pid}/task"));
    if let Err(_) = task {
        return (read_bytes, write_bytes);
    }
    let task = task.unwrap();
    task.filter_map(|v| {
        let path = v.unwrap().path();
        let tid = path.to_str().unwrap();
        let children = read_to_string(format!("{tid}/children"));
        if let Err(_) = children {
            return None;
        }
        Some(
            children
                .unwrap()
                .split_whitespace()
                .map(|v| rwbytes(v.parse::<u32>().unwrap()))
                .fold((0, 0), |(acc1, acc2), (v1, v2)| (acc1 + v1, acc2 + v2)),
        )
    })
    .fold((read_bytes, write_bytes), |(acc1, acc2), (v1, v2)| {
        (acc1 + v1, acc2 + v2)
    })
}
