#![feature(map_try_insert)]

mod output;
mod prof;
mod util;

use std::{
    env,
    error::Error,
    fs::{File, self},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

use output::{Outputter, PlotOutput};
use prof::{Event, PerfStat, Prof};

struct Args {
    freq_millis: u32,
    profs: Vec<Box<dyn Prof>>,
    subprocess: Child,
    perf_flag: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut args_iter = env::args();
    args_iter.next();
    let mut freq_millis: Option<u32> = None;
    let mut profs = Vec::new();
    let mut perf_flag = false;
    let mut perf_events = String::new();
    let help =
        "monitor {--help} | {-f N [-p perf_event1,...] [-e event1,...] -- <program>}".to_string();

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--help" => {
                return Err(help);
            }
            "-f" => match args_iter.next() {
                Some(f) => {
                    freq_millis = Some(f.parse().unwrap());
                }
                None => return Err(help),
            },
            "-p" => match args_iter.next() {
                Some(events) => {
                    perf_flag = true;
                    perf_events = events;
                }
                None => return Err(help),
            },
            "-e" => match args_iter.next() {
                Some(events) => {
                    profs = prof::profs(&events);
                }
                None => return Err(help),
            },
            "--" => {
                break;
            }
            _ => return Err(help),
        }
    }

    let cmd_line = args_iter.collect::<Vec<_>>();
    if cmd_line.len() == 0 || freq_millis == None {
        return Err(help);
    }
    let freq_millis = freq_millis.unwrap();

    let subprocess;
    if perf_flag {
        File::create("perf.pipe").unwrap();
        let mut cmd = Command::new("perf");
        cmd.args([
            "stat",
            "-j",
            "-I",
            &freq_millis.to_string(),
            "-e",
            &perf_events,
            "-o",
            "perf.pipe",
        ]);
        cmd.args(&cmd_line)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        subprocess = cmd.spawn().unwrap();
    } else {
        let mut cmd = Command::new(&cmd_line[0]);
        cmd.args(&cmd_line[1..]).stdout(Stdio::null());
        subprocess = cmd.spawn().unwrap();
    }

    Ok(Args {
        freq_millis,
        profs,
        subprocess,
        perf_flag,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    if !nix::unistd::geteuid().is_root() {
        Err("User must have root privileges")?
    }
    if let Err(_) = fs::read_dir("plots") {
        fs::create_dir("plots")?;
    }

    let (sender, reciever) = mpsc::channel::<Event>();
    let mut args = parse_args()?;
    let pid = args.subprocess.id();
    for prof in args.profs {
        let sender = sender.clone();
        thread::spawn(move || prof.profiler(args.freq_millis, pid, sender));
    }

    args.subprocess.wait()?;
    let mut events = Vec::new();
    while let Ok(event) = reciever.recv_timeout(Duration::from_millis(1)) {
        events.push(event);
    }
    if args.perf_flag {
        let mut perf_events = PerfStat::new(File::open("perf.pipe")?);
        events.append(&mut perf_events);
        std::fs::remove_file("perf.pipe")?;
    }
    events.sort_by_key(|v| v.timestamp_millis);

    PlotOutput::new("plots").output(&events);

    Ok(())
}