#[allow(dead_code, unused)]
/// A small logging tool.
///
/// This is not the logger you're looking for. This logger is
/// basic. It is a toy project and my first attempt to program
/// in rust. Use at your own risk.
///
/// # Examples
///
/// Create a new default logger:
///
/// ```no_run
/// use log::Logger;
///
/// let logger = Logger::new();
///
/// ```
///
/// Write a message to the default log file created during initialization:
///
/// ```no_run
/// logger.log_message("Enter your message here!", None)
/// ```
///
/// Example default output:
/// [LLOG]::[2024-05-21 18:37:22] -> Enter your message here!
///
/// The None in the [self].log_message() method is an optional parameter
/// to allow the user to pass a message and an error.
///
/// Set custom logging options:
/// The set methods displayed below can be used in any combination and order.
/// Default values have already been defined and these methods are convience methods for
/// making defining custom behavior easier.
///
/// ```no_run
///
/// use log::{LoggerOptions, Logger};
///
/// let opts = LoggerOptions::new()
///    .set_log_type("Choose: file, console or both.")
///    .set_logfile_name("Any_valid_file_name_for_your_os")
///    .set_dest_dir("/must/be/absolute/path")
///    .set_log_label("Replace LLOG in above example.")
///    .set_dt_format("%Y-%m-%d);
///
/// let logger = Logger::new(opts);
///
///
/// ```
/// Date time formating is done with the [chrono crate](https://crates.io/crates/chrono).
/// All credit to them for handling what is easily the hardest part of this little project.
/// For a complete list of date/time formatting options see their great
/// [documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

pub mod log {
    use chrono::Local;
    use std::env::{self, set_current_dir};
    use std::fmt;
    use std::fs::{File, OpenOptions};
    use std::io::{prelude::*, ErrorKind, Write};
    use std::path::{Path, PathBuf};

    #[derive(Debug)]
    enum LogType {
        FileOnly,
        ConsoleOnly,
        Both,
    }

    #[derive(Debug)]
    pub struct LoggerOptions {
        log_file_name: String,
        log_type: LogType,
        log_label: String,
        dt_format: String,
        use_dt: bool,
        use_label: bool,
    }

    impl Default for LoggerOptions {
        fn default() -> Self {
            LoggerOptions::new()
        }
    }

    impl LoggerOptions {
        pub fn new() -> Self {
            LoggerOptions {
                log_file_name: String::from("llog.txt"),
                log_type: LogType::FileOnly,
                log_label: String::from("LLOG"),
                dt_format: String::from("%Y-%m-%d %H:%M:%S"),
                use_dt: true,
                use_label: true,
            }
        }
    }

    pub trait Modifiers {
        fn set_log_type(&mut self, log_type: &str);
        fn set_logfile_name(&mut self, new_name: &str);
        fn set_dest_dir(&mut self, new_dest: &str);
        fn set_log_label(&mut self, new_label: &str);
        fn set_dt_format(&mut self, new_format: &str);
    }

    impl Modifiers for LoggerOptions {
        fn set_log_type(&mut self, log_type: &str) {
            self.log_type = match log_type.to_uppercase().as_str() {
                "FILE" | "FILEONLY" => LogType::FileOnly,
                "CONSOLE" | "CONSOLEONLY" => LogType::ConsoleOnly,
                "BOTH" => LogType::Both,
                _ => panic!("Invalid log type provided."),
            }
        }

        /// Does not support symlinks.
        /// new_dest must be an absolute path.
        fn set_dest_dir(&mut self, new_dest: &str) {
            let dest: &Path = Path::new(new_dest);
            assert!(&dest.is_absolute());
            assert!(&dest.is_dir());
            assert!(&dest.exists());
            set_current_dir(dest);
        }

        fn set_logfile_name(&mut self, new_name: &str) {
            self.log_file_name = new_name.to_string();
        }
        fn set_log_label(&mut self, new_label: &str) {
            self.log_label = new_label.to_string();
        }
        fn set_dt_format(&mut self, new_format: &str) {
            self.dt_format = new_format.to_string();
        }
    }

    /// Logger Code
    /// TODO: Add Docstrings
    #[derive(Debug)]
    pub struct Logger {
        msg: String,
        date_time: String,
        dt_format: String,
        log_label: String,
        log_type: LogType,
        use_dt: bool,
        use_label: bool,
        log_file: File,
    }
    trait Internals {
        fn update_time(&mut self);
        fn update_log_line(&mut self, msg: (&str, &str));
    }

    impl Internals for Logger {
        fn update_time(&mut self) {
            self.date_time = Local::now().format(&self.dt_format).to_string();
        }

        fn update_log_line(&mut self, msg: (&str, &str)) {
            self.update_time();
            self.msg = format!(
                "[{}]::[{}] -> {}{}",
                self.log_label, self.date_time, msg.0, msg.1
            );
        }
    }

    impl fmt::Display for Logger {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            writeln!(formatter, "{}", &self.msg)
        }
    }

    impl Default for Logger {
        fn default() -> Self {
            Logger::new(Some(LoggerOptions::new()))
        }
    }

    pub trait LoggerIO {
        fn log_message(&mut self, msg: &str, err: Option<&str>);
    }

    impl LoggerIO for Logger {
        fn log_message(&mut self, msg: &str, err: Option<&str>) {
            /// This is the entry point to the logger. Use it to log messages and errors.
            ///
            /// Use this function to write messages to either the console, the log file or both, depending
            /// on which logger settings you have enabled. This function takes a message and an optional parameter
            /// for passing an error or secondary message to be logged.
            ///
            /// # Examle:
            /// ```no_run
            ///
            /// let message: &str:
            /// let logger = Logger::new();
            ///
            /// ```
            let error = err.unwrap_or("");
            self.update_log_line((msg, error));
            let msg_to_write: &str = self.msg.as_str();
            match &self.log_type {
                LogType::Both => {
                    self.log_file.write_all(msg_to_write.as_bytes());
                    println!("{}", &self);
                }
                LogType::FileOnly => {
                    self.log_file.write_all(msg_to_write.as_bytes());
                }
                LogType::ConsoleOnly => {
                    println!("{}", &self);
                }
            };
        }
    }

    impl Logger {
        pub fn make_log_file(file_name: &String) -> File {
            /// Returns an object with write access to an open file on the filesystem.
            ///
            /// This fuction creates a file in the current working directory of
            /// the project. The location of the destination output file can be
            /// changed to any valid folder on the system with the set_dest_dir() method.
            /// # Example
            ///
            /// ```no_run
            /// let opts = LoggerOptions::new().set_dest_dir("/some/new/file/location");
            /// let logger = Logger::new(opts);
            /// ```
            let name: &str = file_name;
            assert!(!Path::new(name).exists());
            return match OpenOptions::new().write(true).create_new(true).open(name) {
                Ok(file) => file,
                Err(_e) => panic!("Error creating new log file {}\n", _e),
            };
        }
        pub fn new(logger_option: Option<LoggerOptions>) -> Self {
            /// Constructs a new logger with an optional parameter for passing custom behavior.
            ///
            /// The default settings used by calling Logger::new() like in the example code below
            /// will produce messages like the default msg on the next line:
            /// \[LLOG\]::\[2024-05-21 18:37:22\] -> Your message was put here! Your optional error went here.
            ///
            /// # Example:
            ///
            /// ```no_run
            /// let opts =  LoggerOptions::new();
            /// let logger = Logger::new();
            /// ```
            ///
            let opts = logger_option.unwrap_or_default();
            return Logger {
                msg: opts.log_file_name.to_string(),
                date_time: Local::now().format(&opts.dt_format).to_string(),
                dt_format: opts.dt_format.to_string(),
                log_label: opts.log_label.to_string(),
                log_type: opts.log_type,
                log_file: Logger::make_log_file(&opts.log_file_name),
                use_dt: opts.use_dt,
                use_label: opts.use_label,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use crate::log::{Logger, LoggerOptions, Modifiers};
    use std::path;
    use chrono::Local;
    
    #[test]
    fn test_set_logfile_name() {
        let opts = LoggerOptions::new().set_logfile_name("test_log.txt");
        let logger = Logger::new(opts);
        assert!(!path::Path::new("test_log.txt").exists());        
    }
    #[test]
    fn test_set_logfile() {
        
    }
    #[test]
    fn test_set_dest_dir() {}
    #[test]
    fn test_set_log_label() {}
    #[test]
    fn test_set_dt_format() {}
    // #[test]
    // fn test_log_date() {
    //     let logger = Logger::new();
    //     let expected = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    //     assert_eq!(expected, logger.date_time);
    // }

}
