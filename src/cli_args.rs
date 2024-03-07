use clap::{Parser, ValueEnum};

use crate::{
    output::{Outputter, PlotOutput, TextOutput},
    prof::Event,
};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Command line tool for profiling any app.",
    long_about = "Profile any app with list of events you choose and build plots with statistics. Only for Linux. Need root priveleges."
)]
pub struct CliArgs {
    #[arg(short, help = "Sampling frequency in milliseconds")]
    pub freq_millis: u32,
    #[arg(
        short,
        value_delimiter = ',',
        help = "List of events delimited by ','",
        long_help = r#"List of events delimited by ','
Possible values:
    * cpu - CPU. Contains:
        - User time in ticks
        - Kernel time in ticks
        - Usage time in ticks
        - Usage in percentage
    * io - Input/Output. Contains:
        - Read bytes
        - Write bytes
    * mem - System memory
    * maps - Process memory
    * net - Network. Contains:
        - Recieved kbytes
        - Transmitted kbytes"#
    )]
    pub events: Vec<String>,
    #[arg(
        short,
        value_delimiter = ',',
        help = "List of perf events delimited by ','; requires perf"
    )]
    pub perf_events: Vec<String>,
    #[arg(short, value_enum, required = true)]
    pub output: OutputType,
    #[arg(last = true, required = true)]
    pub cmd: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputType {
    PLOTS,
    TEXT,
}

impl Outputter for OutputType {
    fn output(&self, events: &[Event]) {
        match self {
            OutputType::PLOTS => PlotOutput::new("plots").output(events),
            OutputType::TEXT => TextOutput {}.output(events),
        }
    }
}
