extern crate log;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::fmt;

use std::sync::{Mutex};

use log::{LogRecord, LogLevel, LogLevelFilter, SetLoggerError, LogMetadata};
use time::{Tm};

fn timestamp() -> String{
    let now = time::now();
    let mut time_string = match time::strftime("%F %T", &now){
                            Ok(S) => S,
                            Err(_) => "".to_string(),
    };

    let milli_string = now.tm_nsec /1000 /1000;

    let timestamp = fmt::format(format_args!("{}.{:03}", time_string,  milli_string));

    return timestamp;
}

pub struct SimpleFileLogger {
    logfile: Mutex<Option<File>>,
}

impl log::Log for SimpleFileLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {

            let mut buffer = Vec::new();
            write!(&mut buffer, "{} - {} - {}\n", timestamp(), record.level(), record.args());

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

impl SimpleFileLogger {
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
            Box::new(SimpleFileLogger {
                logfile: Mutex::new(Some(file))
            })
        })
    }


}
