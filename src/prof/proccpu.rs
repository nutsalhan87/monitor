use std::{
    fs::{read_dir, read_to_string},
    thread,
    time::{Duration, Instant}, sync::mpsc::Sender,
};

use super::{Event, Prof};

pub struct ProcCpu {}

impl Prof for ProcCpu {
    fn profiler(
        &self,
        freq_millis: u32,
        pid: u32,
        sender: Sender<Event>,
    ) {
        let start_time = Instant::now();
        let (mut utime_prev, mut stime_prev) = ustime(pid);
        let mut cpu_time_prev = cpu_time();
        loop {
            thread::sleep(Duration::from_millis(freq_millis as u64));
            let timestamp_millis = Instant::now().duration_since(start_time).as_millis() as u32;
            let (utime, stime) = ustime(pid);
            let cpu_time = cpu_time();
            let (utime_dif, stime_dif) = (utime - utime_prev, stime - stime_prev);
            let cpu_time_dif = cpu_time - cpu_time_prev;
            sender
                .send(Event {
                    timestamp_millis,
                    description: "user time".to_string(),
                    value: Some(utime_dif),
                    unit: "ticks".to_string()
                })
                .unwrap();
            sender
                .send(Event {
                    timestamp_millis,
                    description: "kernel time".to_string(),
                    value: Some(stime_dif),
                    unit: "ticks".to_string()
                })
                .unwrap();
            sender
                .send(Event {
                    timestamp_millis,
                    description: "usage time".to_string(),
                    value: Some(utime_dif + stime_dif),
                    unit: "ticks".to_string()
                })
                .unwrap();
            if cpu_time_dif != 0 {
                sender
                .send(Event {
                    timestamp_millis,
                    description: "usage".to_string(),
                    value: Some((utime_dif + stime_dif) * 100 / cpu_time_dif),
                    unit: "%".to_string()
                })
                .unwrap();
            }
            utime_prev = utime;
            stime_prev = stime;
            cpu_time_prev = cpu_time;
        }
    }
}

fn cpu_time() -> u64 {
    let stat = read_to_string("/proc/stat").unwrap();
    let line = stat.lines().next().unwrap();
    let count = line
        .split_whitespace()
        .skip(1)
        .take(8)
        .fold(0u64, |acc, v| acc + v.parse::<u64>().unwrap());
    count
}

fn ustime(pid: u32) -> (u64, u64) {
    let stat = read_to_string(format!("/proc/{pid}/stat"));
    if let Err(_) = stat {
        return (0, 0);
    }
    let stat = stat.unwrap();
    let mut words = stat.split_whitespace().skip(13);
    let utime: u64 = words.next().unwrap().parse().unwrap();
    let stime: u64 = words.next().unwrap().parse().unwrap();
    let task = read_dir(format!("/proc/{pid}/task"));
    if let Err(_) = task {
        return (utime, stime);
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
                .map(|v| ustime(v.parse::<u32>().unwrap()))
                .fold((0, 0), |(acc1, acc2), (v1, v2)| (acc1 + v1, acc2 + v2)),
        )
    })
    .fold((utime, stime), |(acc1, acc2), (v1, v2)| {
        (acc1 + v1, acc2 + v2)
    })
}
