use std::{
    fs,
    io::{self, BufWriter, Write},
};

struct AuditLog {
    file: io::BufWriter<fs::File>, // BufWriter batches writes; flush() commits to disk
    closed: bool,                  // tracks whether close() was explicitly called
}

impl AuditLog {
    fn new(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = fs::OpenOptions::new()
            .append(true) // don't truncate on open; preserves prior audit entries
            .create(true) // create file if missing
            .open(path)?;
        Ok(Self {
            file: BufWriter::new(file),
            closed: false,
        })
    }

    fn write_event(&mut self, entry: &str) -> io::Result<()> {
        writeln!(self.file, "{}", entry)?; // buffered, not yet on disk
        Ok(())
    }

    fn close(&mut self) -> io::Result<()> {
        self.file.flush()?; // commits buffer to disk; error surfaces here
        self.closed = true; // only set after successful flush; order matters
        Ok(())
    }
}

impl Drop for AuditLog {
    fn drop(&mut self) {
        if !self.closed {
            eprintln!("WARNING: close() was never called explicitly.");
            // caller forgot explicit close()
            if let Err(e) = self.close() {
                if std::thread::panicking() {
                    eprintln!("{}", e); // already unwinding; don't double panic (causes abort)
                } else {
                    panic!("{}", e); // make the forgotten close() loud in normal flow
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut logger = AuditLog::new("log.txt")?;
    logger.write_event("daemon started")?;
    // logger.close()?; // explicit close; production path, errors handled by caller
    Ok(())
}
