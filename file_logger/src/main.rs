use std::io::Write;

struct FileLogger {
    file: std::fs::File,
    buffer: Vec<String>,
}

impl FileLogger {
    fn new(path: &str) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;
        Ok(Self {
            file,
            buffer: vec![],
        })
    }

    fn log(&mut self, msg: impl Into<String>) {
        // can also do param: msg: String, but this impl gives
        // more freedom to caller, he can either do &str or String, both supported (as shown in
        // main). in case of &str
        // the msg.into() will do an allocation on heap and own the value, but in case of String, there wont be new
        // allocation and the value (here "B") will simply move into this log().
        self.buffer.push(msg.into());
    }
}

impl Drop for FileLogger {
    fn drop(&mut self) {
        for buf in self.buffer.iter() {
            let write = writeln!(self.file, "{}", buf);
            if let Err(e) = write {
                eprintln!("{e}");
            }
        }
        let flush = self.file.flush();
        if let Err(e) = flush {
            eprintln!("{e}");
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut logger = FileLogger::new("file.md")?;

    logger.log("A"); // &str works
    logger.log("B".to_string()); // String works too!
    logger.log("C".to_string());

    // std::mem::drop(logger); // or simply drop(logger)
    // logger.log("D"); // ERROR: logger dropped

    Ok(())
}
