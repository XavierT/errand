extern crate log;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Write;

use std::sync::{Mutex};

use log::{LogRecord, LogLevel, LogLevelFilter, SetLoggerError, LogMetadata};

pub struct SimpleLogger {
    logfile: Mutex<Option<File>>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            // println!("{} - {}", record.level(), record.args());

            let mut buffer = Vec::new();
            write!(&mut buffer, "{} - {}", record.level(), record.args());

            let mut file = self.logfile.lock().unwrap();

            match *file {
                Some(ref mut file) => {
                    match file.write(buffer.as_slice()) {
                        Err(why) => panic!("couldn't write: {}", Error::description(&why)),
                        Ok(_) => (),
                    }
                }
                None => panic!("logfile does not exist!"),
            };
        }
    }
}

impl SimpleLogger {
    pub fn init() -> Result<(), SetLoggerError> {

        let path = Path::new("trace.log");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };


        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(SimpleLogger {
                logfile: Mutex::new(Some(file))
            })
        })
    }
}
