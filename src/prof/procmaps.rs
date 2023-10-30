use std::{
    fs::{read_dir, read_to_string},
    thread,
    time::{Duration, Instant}, sync::mpsc::Sender,
};

use super::{Event, Prof};

pub struct ProcMaps {}

impl Prof for ProcMaps {
    fn profiler(
        &self,
        freq_millis: u32,
        pid: u32,
        sender: Sender<Event>,
    ) {
        let start_time = Instant::now();
        loop {
            thread::sleep(Duration::from_millis(freq_millis as u64));
            let timestamp_millis = Instant::now().duration_since(start_time).as_millis() as u32;
            let value = maps(pid);
            sender
                .send(Event {
                    timestamp_millis,
                    description: "Process memory".to_string(),
                    value: Some(value),
                    unit: "kB".to_string()
                })
                .unwrap();
        }
    }
}

fn maps(pid: u32) -> u64 {
    let maps_file = read_to_string(format!("/proc/{pid}/maps"));
    if let Err(_) = maps_file {
        return 0;
    }
    let maps_file = maps_file.unwrap();
    let mem = maps_mem(&maps_file);
    let task = read_dir(format!("/proc/{pid}/task"));
    if let Err(_) = task {
        return mem;
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
                .map(|v| maps(v.parse::<u32>().unwrap()))
                .fold(0, |acc, v| acc + v),
        )
    })
    .fold(mem, |acc, v| acc + v)
}

fn maps_mem(maps: &str) -> u64 {
    maps.lines()
        .filter_map(|v| {
            let mut words = v.split_whitespace();
            let adress = words.next().unwrap();
            let flags = words.next().unwrap();
            if flags.as_bytes()[1] == b'w' && flags.as_bytes()[3] == b'p' {
                let mut mapped = adress
                    .split('-')
                    .map(|v| u64::from_str_radix(v, 16).unwrap());
                let start = mapped.next().unwrap();
                let end = mapped.next().unwrap();
                Some(end - start)
            } else {
                None
            }
        })
        .sum()
}
