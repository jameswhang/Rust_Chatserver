#[doc="
    @authors: James Whang (syw973) and Adel Lahlou (adl538)

    usage: nc localhost 8080
    GET Cargo.toml HTTP/1.1

    Our webserver uses the Rust standard library except for the chrono crate which is used for
    generating timestamps for loggger. Our code is organized into the server operations, which
    are found here in main.rs. The server depends on our http and logger modules.

    Http module exposes two structs (HttpRequest and HttpResponse) and one enum (HttpStatusCode).
    There was opportunity to go further with a parser, etc. but full support of HTTP was outside
    the scope of the homework specifications. These structs expose useful static functions that
    allow us to generate HttpRequest objects from a String, and HttpResponse objects from an
    HttpRequest object and its payload.

    The logger module exposes HttpLogger which has one field of type Arc<Mutex<File>>. File was
    chosen for convenience. An HttpLogger is initiliazed with a file path for the logfile, and
    exposes a three methods to log, log_request, log_response, and log_request_response. These
    methods use a helper function to generate the log string then call the instance method write.
    Write acquires the lock to the file and then writes into the file

    The server logic is present in main. It creates a logfile in the current directory with the
    filename log.txt, overwriting any previous log.txt. It creates a listening socket on port
    8080 (cuz this is an internet server duh) and accepts all possible connections. Once a
    connection is accepted, a thread is spawned and enters handle client which implements a basic,
    non-persistent HTTP connection. Note that each IO operation is blocking, meaning the thread will
    wait to read from the socket, read from the file, and write into the socket. While responding
    to a request, the details are logged into log.txt and after responding, the connection is closed.


    Comments :
        - Blocking I/O for read, write files/sockets. Because of threads, a smart scheduler can
        reduce the blocking penalty but still have to pay the scheduling/switching penalty
        - Such a server (per the homework spec) pays the thread overhead penalty
        - Would not be a bad idea to have a thread pool and assign them streams to handle
        - Not a bad idea to limit number of connections

    Assumptions:
        - We don't care what version of HTTP it is, even if it is non-existent (HTTP/2.0), as long
        as it is HTTP we'll accept
        - We will respond saying we use the same protocol (because that's what a real server would do)
        but we don't actually
        - We only handle the GET method. Anything else is an error
        - We don't handle request headers
        - All bad http requests are responded to with HTTP/1.1
        - We don't have to limit number of connections
        - If server process cant access file, then it's forbidden to user (403)
"]
//extern crate webserver;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{ErrorKind, Read, Write};
use std::fs::File;
use std::env;

mod http;
mod httplogger;
use self::http::{HttpRequest, HttpResponse, HttpStatusCode};
use self::httplogger::{HttpLogger};



/// Starts a TCPListener that will accept connections and if successful
/// process them and spawn a new thread for each TcpStream
/// Each thread is given access to an HttpLogger the logs to the same file with mutex access
pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let logger = HttpLogger::new("log.txt");

    println!("Started adl538-syw973 server!");

    for stream in listener.incoming() {
        // connection succeeded
        let mut stream = stream.unwrap();
        let log_lock = logger.clone();

        thread::spawn(move || {
            handle_client(&mut stream, &log_lock)
        });
    }
}

/// Called by listener to handle each connection. Reads the http request from the stream,
/// generates the proper http response, and logs all actions. Logging is sychronous (mutex)
///
/// @param stream : &TcpStream
/// @param log_lock : HttpLogger
fn handle_client(stream: &mut TcpStream, logger: &HttpLogger) {
    println!("handling: {:?}", stream.peer_addr().unwrap());
    let raw_http_request = read_http_request(stream); // blocking
    let mut req : HttpRequest = HttpRequest::new_from(raw_http_request);
    let resp: HttpResponse;

    if req.get_status() == HttpStatusCode::BadHttpRequest {
        resp = HttpResponse::new_from(&req, "".to_string());
    } else {
        let file_contents = read_file(req.get_path().clone()); // blocking

        match file_contents {
            Ok(payload) => {
                req.set_status(HttpStatusCode::OK);
                resp = HttpResponse::new_from(&req, payload);
            },

            Err(err_code) => {
                if err_code == ErrorKind::NotFound {
                    req.set_status(HttpStatusCode::NotFound);
                } else if err_code == ErrorKind::PermissionDenied {
                    req.set_status(HttpStatusCode::Forbidden);
                }

                resp = HttpResponse::new_from(&req, "".to_string());
            }
        }
    }

    logger.log_request_response(&req, &resp);
    send_response(stream, &resp);
}

/// Reads from TcpStream. Blocks until completely read
/// @param stream : &TcpStream
///
/// @return String returns the raw HTTP request as a String
fn read_http_request(stream: &mut TcpStream) -> String {
    const BUF_SIZE: usize = 1024;
    let mut buf = [0; BUF_SIZE];
    let mut result = String::new();
    let mut addition: String;

    // continually pass in a buffer until nothing left to read
    while let Ok(length) = stream.read(&mut buf[..]) {
        // add data in buffer to results string
        addition = String::from_utf8(buf.to_owned()).unwrap();
        result.push_str(&addition);
        buf = [0; BUF_SIZE];

        // break if all of input has been read
        if length < BUF_SIZE {
            break;
        }
    }
    return result;
}



/// Reads from file if it exists at provided path. Blocks until completely read.
/// @param file_path : String
///
/// @return Result<String, ErrorKind> returns a result with the file contents or an Error
fn read_file(file_path: String) -> Result<String, ErrorKind>{
    let mut server_path = env::current_dir().unwrap();
    server_path.push(file_path);

    let file = File::open(server_path);

    match file {
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content);
            Ok(file_content)
        }
        Err(e) => {
            Err(e.kind())
        }
    }
}


/// Blocking. Sends HttpResponse into stream
///
/// @param stream : &mut TcpStream
/// @param resp : &HttpResponse
fn send_response(stream: &mut TcpStream, response: &HttpResponse) {
    println!("", );
    let mut response_text = format!("\n{} {} {}\n",
                                        response.get_protocol(),
                                        response.get_status() as usize,
                                        HttpResponse::get_status_tag(response.get_status())
                                    );

    if response.get_status() == HttpStatusCode::OK {
        response_text = response_text +
            &format!(" Server: adl538-syw973-webserver\n Content-type: {}\n Content-length: {}\n\n\n{}\n\n",
                            response.get_content_type(),
                            response.get_content_length(),
                            response.get_payload()
                        );
    }

    stream.write(response_text.as_bytes());
}
