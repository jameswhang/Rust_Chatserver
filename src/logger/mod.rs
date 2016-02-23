extern crate chrono;
// extern crate webserver;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

use super::http::*;
use self::chrono::*;


type LogfileLock = Arc<Mutex<File>>;

#[derive(Clone)]
pub struct Logger(LogfileLock);

impl Logger {
    pub fn new(filepath : &str) ->HttpLogger{
        match File::create(filepath) {
            Ok(logfile) => {
                Logger(Arc::new(Mutex::new(logfile)))
            },

            Err(_) => {
                panic!("Log file creation failed");
            }
        }
    }

    pub fn write(&self, data : &[u8]) {
        let mut logfile = self.0.lock().unwrap();
        logfile.write(data);
    }
}
