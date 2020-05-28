use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // dd if=/dev/urandom bs=1024 count=128 of=myfile
    // cat myfile | target/debug/pipeviewer > myfile2

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        eprintln!("{}", total_bytes);
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

    eprintln!("{}", total_bytes);

    Ok(())
}

/*
.git/hooks/pre-commit

cargo fmt
exec cargo clippy -- -D warnings
*/
