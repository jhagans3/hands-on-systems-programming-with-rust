use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    // dd if=/dev/urandom bs=1024 count=128 of=myfile
    // cat myfile | target/debug/pipeviewer > myfile2
    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;

        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    eprintln!("total bytes: {}", total_bytes);
}

/*
.git/hooks/pre-commit

cargo fmt
exec cargo clippy -- -D warnings
*/
