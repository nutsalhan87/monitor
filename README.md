# Monitor
## Command line tool for profiling any app

Profile any app with list of events you choose and build plots with statistics.
Only for Linux. Need root priveleges.

## Using

`monitor {--help} | {-f N [-p perf_event1,...] [-e event1,...] -- <program>}`
Where
- `--help` -- help
- `-f N` -- N is frequency of statistics collecting in milliseconds
- `-p perf_event1,...` -- list of perf events without spaces separated by commas. When you specify this key, minotor running perf tool so you need it installed on your system
- `-e event1,...` -- list of built-in events without spaces separated by commas.
  List of build-in events:
  - maps -- memory for process
  - mem -- memory for whole system
  - io -- read/write bytes for processu
  - cpu -- user/kernel/usage time for process
  - net -- recieve/transmit for every net devies for process (works bad)
- `<program>` -- any program with its arguments. Not a string

## Building
Requerments:
- perf tool for -p key
- rustc, cargo
- pkg-config
- fontconfig
- freetype

Build with `cargo build --release`

# Using
Needs directory named `plots` in the program path. Plots will be saved there.