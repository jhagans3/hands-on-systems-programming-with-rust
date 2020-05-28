use std::io::{self, ErrorKind, Read, Result, Write};
use structopt::StructOpt;

const CHUNK_SIZE: usize = 16 * 1024;

// cargo run -- --help
#[derive(StructOpt, Debug)]
// #[structopt(name = "basic")]
struct Opt {
    /// Read from a file instead of stdin
    #[structopt()]
    infile: Option<String>,

    /// Write output to a file instead of stdout
    #[structopt(short, long)]
    outfile: Option<String>,

    /// Display total bytes
    #[structopt(short, long)]
    silent: bool,
}

fn main() -> Result<()> {
    // dd if=/dev/urandom bs=1024 count=128 of=myfile
    // cat myfile | target/debug/pipeviewer > myfile2

    let args = Opt::from_args();
    let silent = args.silent;

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        if !silent {
            eprintln!("{}", total_bytes);
        }

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            // yes | cargo run | head -n 1 > /dev/null
            // Error: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

            // skip broken pipe error
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);

            // yes | cargo run | head -n 2 > /dev/null
            // Oh no, an error! Broken pipe (os error 32)
            // eprint!("Oh no, an error! {}\n", e.to_string());
            // std::process::exit(1);
        }
    }

    if !silent {
        eprintln!("total bytes: {}", total_bytes);
    }

    Ok(())
}

/*
.git/hooks/pre-commit

cargo fmt
exec cargo clippy -- -D warnings
*/
