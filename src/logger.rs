extern crate chrono;
// extern crate webserver;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

use super::http::*;
use self::chrono::*;


type LogfileLock = Arc<Mutex<File>>;

#[derive(Clone)]
pub struct HttpLogger(LogfileLock);

impl HttpLogger {
    pub fn new(filepath : &str) ->HttpLogger{
        match File::create(filepath) {
            Ok(logfile) => {
                HttpLogger(Arc::new(Mutex::new(logfile)))
            },

            Err(_) => {
                panic!("Log file creation failed");
            }
        }
    }

    /// Helper function to write into log file
    ///
    /// @param req : &HttpRequest
    /// @param resp : &HttpResponse
    pub fn log_request_response(&self, req : &HttpRequest, resp : &HttpResponse) {
        let req_str = HttpLogger::gen_request_log(req);
        let resp_str = HttpLogger::gen_response_log(resp);
        self.write( format!("{}\n{}\n", req_str, resp_str).as_bytes());
    }

    /// Helper function to write into log file
    ///
    /// @param req : &HttpRequest
    pub fn log_request(&self, req : &HttpRequest) {
        let req_str = HttpLogger::gen_request_log(req);
        self.write( format!("{}\n", req_str).as_bytes());
    }

    /// Helper function to write into log file
    ///
    /// @param req : &HttpResponse
    pub fn log_response(&self, resp : &HttpResponse) {
        let resp_str = HttpLogger::gen_response_log(resp);
        self.write( format!("{}\n", resp_str).as_bytes());
    }


    fn write(&self, data : &[u8]) {
        let mut logfile = self.0.lock().unwrap();
        logfile.write(data);
    }

    fn gen_request_log(req: &HttpRequest) -> String {
        let date = &Local::now().format("%m-%d-%Y %H:%M:%S").to_string();
        format!("{date}: REQUEST - {request_method} {request_path}",
                                                        date=date,
                                                        request_method=req.get_method(),
                                                        request_path=req.get_path())
    }

    fn gen_response_log(resp: &HttpResponse) -> String {
        let date = &Local::now().format("%m-%d-%Y %H:%M:%S").to_string();
        format!("{date}: RESPONSE - {status_code} {tag} {content_type}",
                                                        date=date,
                                                        status_code=resp.get_status() as usize,
                                                        tag=HttpResponse::get_status_tag(resp.get_status()),
                                                        content_type=resp.get_content_type())
    }


}
