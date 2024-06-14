#[allow(dead_code, unused)]
/// My logging tool.
///
/// This is not the logger you're looking for. This logger is
/// basic. It is a toy project and my first attempt to program
/// in rust. Use at your own risk.
///
/// Date time formating is done with the [chrono crate](https://crates.io/crates/chrono).
/// All credit to them for handling what is easily the hardest part of this little project.
/// For a complete list of date/time formatting options see their great
/// [documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).
///
/////////////////////////////////////////////////////////////////////////////////////////////
/// # Examples
///
/// Create the default logger preconfigured with these settings:
///
///    File name: llog.txt
///    Log type: file only.
///    Log label: \[LLOG\]
///    Date/Time format: 2024-05-21 18:37:22
///    Use date/time: Yes
///    Use label: Yes
///
/// Write a message to the log file created during initialization:
///
/// ```no_run
/// use little_logger::log;
///
/// let mut logger = log::Logger::default();
///
/// logger.log_message("Enter your message here!");
///
/// ```
///
/// Output to llog.txt:
/// ```no_run
/// [LLOG]::[2024-05-21 18:37:22] -> Enter your message here!
/// ```
///
/////////////////////////////////////////////////////////////////////////////////////////////
/// # Examples
/// Set custom logging options:
///
/// The set methods displayed below can be used with the standard Rust
/// method chaining paradigm.
///
/// ```no_run
/// use little_logger::log::{Logger, LoggerOpts};
///
/// let mut opts = LoggerOpts::new()
///    .set_log_type("both")
///    .set_logfile_name("my_log")
///    .set_dest_dir("/home/me/logfiles")
///    .set_log_label("SERVER")
///    .set_dt_format("%H-%M-%S);
/// ```
/// Create a new logger with the above options and log a message:
///
/// ```no_run
/// let mut logger = Logger::new(opts);
///
/// logger.log_message("Enter your message here!");
/// ```
///
/// Example output to my_log:
///
///     [SERVER]::[18:37:22] -> Enter your message here!
///
/// Example output to console:
///
///     [SERVER]::[18:37:22] -> Enter your message here!
///
///

pub mod log {

    use chrono::Local;
    use std::env::set_current_dir;
    use std::fs::{File, OpenOptions};
    use std::io::{prelude::*, StdoutLock, Write};
    use std::path::{Path, PathBuf};
    use std::{fmt, io};

    #[derive(Debug)]
    enum LogType<'a> {
        File(Box<LogFile>),
        Console(Box<LogConsl<'a>>),
        Both(Box<LogConsl<'a>>, Box<LogFile>),
    }

    #[derive(Debug)]
    struct LogFile {
        out: File,
    }

    impl LogFile {
        fn new(file_name: &str) -> Box<LogFile> {
            let mut file = OpenOptions::new().append(true).open(file_name);
            let mut logfile: LogFile = LogFile {
                out: file.expect("Failed to open log file"),
            };
            Box::new(logfile)
        }
    }

    #[derive(Debug)]
    struct LogConsl<'a> {
        out: StdoutLock<'a>,
    }

    impl<'a> LogConsl<'a> {
        fn new() -> Box<LogConsl<'a>> {
            let mut console: LogConsl<'a> = LogConsl {
                out: io::stdout().lock(),
            };
            Box::new(console)
        }
    }

    #[derive(Debug)]
    pub struct LoggerOpts<'a> {
        log_file_name: String,
        log_type: LogType<'a>,
        log_label: String,
        dt_format: String,
        use_dt: bool,
        use_label: bool,
    }

    impl<'a> LoggerOpts<'a> {
        pub fn new() -> LoggerOpts<'a> {
            LoggerOpts {
                log_file_name: String::from("llog.txt"),
                log_type: LogType::File(LogFile::new("llog.txt")),
                log_label: String::from("LLOG"),
                dt_format: String::from("%Y-%m-%d %H:%M:%S"),
                use_dt: true,
                use_label: true,
            }
        }
        /// Define where log messages are written.
        ///
        /// Three options dictate how a logger instance will write messages.
        /// File, Console, and Both. The names are self explanitory.
        /// The LoggerOpts default is to write to a file. This method is used
        /// to change that setting.
        pub fn set_log_type(mut self, log_type: &str) -> Self {
            self.log_type = match log_type.to_uppercase().as_str() {
                "FILE" | "FILEONLY" => LogType::File(LogFile::new("llog.txt")),
                "CONSOLE" | "CONSOLEONLY" => LogType::Console(LogConsl::new()),
                "BOTH" => LogType::Both(LogConsl::new(), LogFile::new("llog.txt")),
                _ => panic!("Invalid log type provided."),
            };
            self
        }

        /// Redefine the output path for the logger file.
        /// If your program relies on you being in a specific path be sure
        /// to change this setting BEFORE moving to that path or change back afterward.
        ///
        /// This method uses std::env::set_current_dir() to change the current directory.
        ///
        /// DOES NOT SUPPORT SYMLINKS.
        pub fn set_dest_dir(mut self, new_dest: &str) -> Self{
            let dest: &Path = Path::new(new_dest);
            assert!(&dest.is_absolute());
            assert!(&dest.is_dir());
            assert!(&dest.exists());
            set_current_dir(dest);
            self
        }
        /// Redefine the name of the log file.
        ///
        /// Default: llog.txt
        pub fn set_logfile_name(mut self, new_name: &str) -> Self {
            self.log_file_name = new_name.to_string();
            self
        }
        /// Redefine the label at the beginning of the log line.
        ///
        /// Default: LLOG
        pub fn set_log_label(mut self, new_label: &str) -> Self {
            self.log_label = new_label.to_string();
            self
        }
        /// Redefine the date/time display settings.
        ///
        /// Default: 2024-05-21 18:37:22
        pub fn set_dt_format(mut self, new_format: &str) -> Self {
            self.dt_format = new_format.to_string();
            self
        }
    }

    #[derive(Debug)]
    pub struct Logger<'a> {
        msg: String,
        date_time: String,
        dt_format: String,
        log_label: String,
        log_type: LogType<'a>,
        use_dt: bool,
        use_label: bool,
    }

    impl<'a> fmt::Display for Logger<'a> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            writeln!(formatter, "{}", &self.msg)
        }
    }

    impl<'a> Default for Logger<'a> {
        fn default() -> Logger<'a> {
            /// Construct the default logger with predefined options.
            ///
            /// # Example:
            ///
            /// ```no_run
            /// use little_logger::log;
            ///
            /// let logger = log::Logger::default();
            /// ```
            ///
            let opt = LoggerOpts::new();
            let opts = opt.set_log_type("file");
            Logger {
                date_time: Local::now().format(&opts.dt_format).to_string(),
                msg: opts.log_file_name.to_string(),
                dt_format: opts.dt_format.to_string(),
                log_label: opts.log_label.to_string(),
                log_type: opts.log_type,
                use_dt: opts.use_dt,
                use_label: opts.use_label,
            }
        }
    }

    impl<'a> Logger<'a> {
        pub fn new(opts: LoggerOpts) -> Logger {
            /// Construct a new logger with custom options.
            ///
            /// # Example:
            ///
            /// ```no_run
            /// use little_logger::log;
            ///
            /// let mut opts =  LoggerOptions::new();
            /// let logger = log::Logger::new();
            /// ```
            ///
            Logger {
                date_time: Local::now().format(&opts.dt_format).to_string(),
                msg: opts.log_file_name.to_string(),
                dt_format: opts.dt_format.to_string(),
                log_label: opts.log_label.to_string(),
                log_type: opts.log_type,
                use_dt: opts.use_dt,
                use_label: opts.use_label,
            }
        }

        fn update_time(&mut self) {
            self.date_time = Local::now().format(&self.dt_format).to_string();
        }

        fn update_log_line(&mut self, msg: (&str, &str)) {
            self.update_time();
            self.msg = format!(
                "[{}]::[{}] -> {}\n{}",
                self.log_label, self.date_time, msg.0, msg.1
            );
        }

        pub fn log_message<S: Into<&'a str>>(mut self, msg: S) {
            /// Use this function to log messages.
            ///
            /// How you define log::LoggerOpts defines whether you log to the console,
            /// a file, or both. If you didn't this will default to a file named llog.txt.
            ///
            /// # Examle:
            /// ```no_run
            ///
            /// let message: &str = "Message to be logged";
            /// let mut logger = Logger::new();
            ///
            /// logger.log_message(message, None);
            ///
            /// ```
            
            self.update_log_line((msg.into(), ""));
            let msg_to_write: &str = self.msg.as_str();
            match self.log_type {
                LogType::Both(ref mut file, ref mut console) => {
                    file.out.write_all(msg_to_write.as_bytes());
                    console.out.write_all(msg_to_write.as_bytes());
                }
                LogType::File(ref mut file) => {
                    file.out.write_all(msg_to_write.as_bytes());
                }
                LogType::Console(ref mut console) => {
                    console.out.write_all(msg_to_write.as_bytes());
                }
            };
        }

        pub fn log_msg_and_error<S: Into<String>>(mut self, msg: S, err: S) {
            /// Use this function to log messages and include an error.
            ///
            /// Msg and err can both be passed as String or &str.
            ///
            /// How you define log::LoggerOpts defines whether you log to the console,
            /// a file, or both. If you didn't this will default to a file named llog.txt.
            ///
            /// # Examle:
            /// ```no_run
            ///
            /// let message: &str = "Message to be logged";
            /// let mut logger = Logger::new();
            /// let err = match some_result = {
            ///      Ok(val) => val,
            ///      Err(e) => e,
            /// }
            ///
            /// logger.log_message(message, err);
            ///
            /// ```
            
            self.update_log_line((&msg.into(), &err.into()));
            let msg_to_write: &str = self.msg.as_str();
            match self.log_type {
                LogType::Both(ref mut file, ref mut console) => {
                    file.out.write_all(msg_to_write.as_bytes());
                    console.out.write_all(msg_to_write.as_bytes());
                }
                LogType::File(ref mut file) => {
                    file.out.write_all(msg_to_write.as_bytes());
                }
                LogType::Console(ref mut console) => {
                    console.out.write_all(msg_to_write.as_bytes());
                }
            };
        }

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_default_opts() {
        
    }

}
