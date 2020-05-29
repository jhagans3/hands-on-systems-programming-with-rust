use pipeviewer2::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

// echo "apple" | cargo run --bin pipeviewer2 -- -o fruit.txt
// echo "hello there" | cargo run --bin pipeviewer2
// echo "hello there" | cargo run --bin pipeviewer2 -- -s
// yes | cargo run --bin pipeviewer | head -n 10000000 > /dev/null
fn main() -> Result<()> {
    let args = Args::get_args();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let quit = Arc::new(Mutex::new(false));
    let (quit1, quit2, quit3) = (quit.clone(), quit.clone(), quit.clone());

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit1));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit2));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit3));

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
