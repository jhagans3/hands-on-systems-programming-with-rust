use pipeviewer2::{args::Opt, read, stats, write};
use std::io::Result;
use structopt::StructOpt;

// echo "apple" | cargo run --bin pipeviewer2 -- -o fruit.txt
// echo "hello there" | cargo run --bin pipeviewer2
// echo "hello there" | cargo run --bin pipeviewer2 -- -s
// yes | cargo run --bin pipeviewer | head -n 10000000 > /dev/null
fn main() -> Result<()> {
    let args = Opt::from_args();
    let mut total_bytes = 0;

    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };

        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }

    stats::stats(args.silent, 0, &mut total_bytes, true);

    Ok(())
}
