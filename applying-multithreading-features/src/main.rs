use pipeviewer2::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::mpsc;
use std::thread;

// echo "apple" | cargo run --bin pipeviewer2 -- -o fruit.txt
// echo "hello there" | cargo run --bin pipeviewer2
// echo "hello there" | cargo run --bin pipeviewer2 -- -s
// yes | cargo run --bin pipeviewer | head -n 10000000 > /dev/null
// yes | cargo run --bin pipeviewer2 -- -o /dev/null

fn main() -> Result<()> {
    let args = Args::get_args();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    //crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any threads return an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
