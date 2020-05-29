use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write(outfile: &Option<String>, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = match outfile {
        Some(outfile) => Box::new(BufWriter::new(File::create(outfile)?)),
        _ => Box::new(BufWriter::new(io::stdout())),
    };

    if let Err(e) = writer.write_all(&buffer) {
        // skip broken pipe error
        if e.kind() == ErrorKind::BrokenPipe {
            // false means "stop the program cleanly"
            return Ok(false);
        }
        return Err(e);
    }

    // true means "keep going"
    Ok(true)
}
