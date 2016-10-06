
/// "Seriousness" of the message
pub enum Level {
    /// Default level
    Normal,
    /// yellow, serious
    Warning,
    /// red !, critical danger
    Danger,
}

/// A single log entry
struct Log {
    level: Level,
    msg: String,
}

/// The whole log history
/// The most recent log is appended as the end of the vector
struct LogBook {
    history: Vec<Log>,
}

impl LogBook {
    pub fn new() -> LogBook {
        LogBook { history: Vec::new() }
    }
}

/// Allow to add new logs
struct Logger {
    book: LogBook,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { book: LogBook::new() }
    }

    pub fn log(&mut self, msg: String) {
        &self.book.history.push(Log {
            level: Level::Normal,
            msg: msg,
        });
    }

    pub fn warn(&mut self, msg: String) {
        &self.book.history.push(Log {
            level: Level::Warning,
            msg: msg,
        });
    }

    pub fn alert(&mut self, msg: String) {
        &self.book.history.push(Log {
            level: Level::Danger,
            msg: msg,
        });
    }
}
