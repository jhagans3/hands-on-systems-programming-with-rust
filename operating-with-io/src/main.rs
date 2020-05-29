use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};
use structopt::StructOpt;

const CHUNK_SIZE: usize = 16 * 1024;

// cargo run --bin pipeviewer -- --help
#[derive(StructOpt, Debug)]
#[structopt(name = "Pipe Viewer")]
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
    // println!("{:#?}", args);
    let silent = args.silent;

    // echo "hello" | cargo run --bin pipeviewer -- > /dev/null
    // echo "hello" | cargo run --bin pipeviewer -- -o /dev/null
    // echo "hello" | cargo run --bin pipeviewer -- -o hello.txt
    // cargo run --bin pipeviewer -- hello.txt -o /dev/null
    // cargo run --bin pipeviewer -- hello.txt -s
    // yes | cargo run --bin pipeviewer -- -o yes.txt
    // cargo run --bin pipeviewer -- yes.txt -o /dev/null
    // cargo run --bin pipeviewer -- yes.txt -o yes2.txt
    // cat yes2.txt | cargo run --bin pipeviewer -- > yes3.txt

    let mut reader: Box<dyn Read> = match args.infile {
        Some(infile) => Box::new(BufReader::new(File::open(infile)?)),
        _ => Box::new(BufReader::new(io::stdin())),
    };

    let mut writer: Box<dyn Write> = match args.outfile {
        Some(outfile) => Box::new(BufWriter::new(File::create(outfile)?)),
        _ => Box::new(BufWriter::new(io::stdout())),
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        if !silent {
            eprint!("\r{}", total_bytes);
        }

        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            // yes | cargo run --bin pipeviewer | head -n 1 > /dev/null
            // Error: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }

            // skip broken pipe error
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);

            // yes | cargo run --bin pipeviewer | head -n 2 > /dev/null
            // Oh no, an error! Broken pipe (os error 32)
            // eprint!("Oh no, an error! {}\n", e.to_string());
            // std::process::exit(1);
        }
    }

    if !silent {
        eprintln!("\r{}", total_bytes);
    }

    Ok(())
}

/*
.git/hooks/pre-commit

cargo fmt
exec cargo clippy -- -D warnings
*/
