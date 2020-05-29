use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::{Arc, Mutex};

pub fn write_loop(outfile: &Option<String>, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer: Box<dyn Write> = match outfile {
        Some(outfile) => Box::new(BufWriter::new(File::create(outfile)?)),
        _ => Box::new(BufWriter::new(io::stdout())),
    };

    loop {
        let buffer: Vec<u8> = Vec::new();
        {
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
        }
        if let Err(e) = writer.write_all(&buffer) {
            // skip broken pipe error
            if e.kind() == ErrorKind::BrokenPipe {
                // stop the program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
