mod cli_args;
mod output;
mod prof;
mod util;

use clap::Parser;
use cli_args::{CliArgs, OutputType};
use output::Outputter;
use prof::{Event, PerfStat, Prof};
use std::{
    error::Error,
    process::{Child, Command, Stdio},
    sync::{mpsc, Arc, RwLock},
    thread,
};

struct Args {
    freq_millis: u32,
    profs: Vec<Box<dyn Prof>>,
    subprocess: Child,
    perf_flag: bool,
    output: OutputType,
}

impl From<CliArgs> for Args {
    fn from(value: CliArgs) -> Self {
        let profs = prof::profs(value.events);
        let perf_flag = !value.perf_events.is_empty();

        let mut cmd;
        if perf_flag {
            cmd = Command::new("perf");
            cmd.args([
                "stat",
                "-j",
                "-I",
                &value.freq_millis.to_string(),
                "-e",
                &value.perf_events.join(","),
            ]);
            cmd.args(value.cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::piped());
        } else {
            cmd = Command::new(&value.cmd[0]);
            cmd.args(&value.cmd[1..]).stdout(Stdio::null());
        }
        let subprocess = cmd.spawn().unwrap();

        Args {
            freq_millis: value.freq_millis,
            profs,
            subprocess,
            perf_flag,
            output: value.output,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = CliArgs::parse();
    if !nix::unistd::geteuid().is_root() {
        Err("User must have root privileges")?
    }

    let args: Args = cli_args.into();
    let (sender, reciever) = mpsc::channel::<Event>();
    let is_program_end = Arc::new(RwLock::new(false));
    let pid = args.subprocess.id();

    for prof in args.profs {
        let sender = sender.clone();
        let is_program_end = is_program_end.clone();
        thread::spawn(move || prof.profiler(args.freq_millis, pid, sender, is_program_end));
    }

    let output = args.subprocess.wait_with_output()?;
    {
        *is_program_end.write().unwrap() = true;
    }

    if !output.status.success() {
        eprintln!("{}", String::from_utf8(output.stderr)?);
        return Ok(());
    }

    let mut events = Vec::new();
    while let Ok(event) = reciever.try_recv() {
        events.push(event);
    }

    if args.perf_flag {
        let mut perf_events = PerfStat::parse_events(std::str::from_utf8(&output.stderr)?);
        events.append(&mut perf_events);
    }
    events.sort_by_key(|v| v.timestamp_millis);

    args.output.output(&events);

    Ok(())
}
