# Monitor
## Command line tool for profiling any app

Profile any app with list of events you choose and build plots with statistics.
Only for Linux. Need root priveleges.

## Using

`monitor [OPTIONS] -f <FREQ_MILLIS> -o <OUTPUT> -- <CMD>...`
- Arguments:
  - \<CMD\>...

- Options:
  - `-f` \<FREQ_MILLIS\>\
    Sampling frequency in milliseconds

  - `-e` \<EVENTS\>\
    List of events delimited by ','\
    Possible values:
    * `cpu` - CPU. Contains:
        - User time in ticks
        - Kernel time in ticks
        - Usage time in ticks
        - Usage in percentage
    * `io` - Input/Output. Contains:
        - Read bytes
        - Write bytes
    * `mem` - System memory
    * `maps` - Process memory
    * `net` - Network. Contains:
        - Recieved kbytes
        - Transmitted kbytes

  - `-p` \<PERF_EVENTS\>\
    List of perf events delimited by ','; requires perf

  - `-o` \<OUTPUT\>\
    [possible values: plots, text]

  - `-h`, --help\
    Print help (see a summary with '-h')

  - `-V`, --version\
    Print version

Plots will be saved in the directory `plots` in the program path.

## Building
Requirements:
- perf tool for -p key
- rustc, cargo
- pkg-config
- fontconfig
- freetype

Build with `cargo build --release`
