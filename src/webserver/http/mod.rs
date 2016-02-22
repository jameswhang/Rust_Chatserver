#[doc="


"]

/// Struct to hold parsed data from HTTP Request
/// Status codes here are used to hold status. Status is in progress until a response is generated
/// Used instead of Result with error because bad requests should
/// still be saved and logged
#[derive(PartialEq, Debug)]
pub struct HttpRequest {
    method: String,
    request_path: String,
    protocol: String,
    status: HttpStatusCode,
}

/// Struct to hold data being assembled for HTTP Response
#[derive(PartialEq, Debug)]
pub struct HttpResponse {
    protocol: String,
    method: String,
    status: HttpStatusCode,
    content_type: String,
    content_length: usize,
    payload: String,
}

/// A few of the relevant HttpStatusCodes. The ones we want to handle
#[derive(Clone, PartialEq, Debug)]
pub enum HttpStatusCode {
    Nil,
    OK = 200,
    BadHttpRequest = 400,
    Forbidden = 403,
    NotFound = 404,
}


impl HttpRequest {
    /// Parses string from raw string from tcpStream into HttpRequest struct
    /// A HTTPRequest object is returned instead of an option that way we can
    /// respond to errors simply.
    ///
    /// @param stream : String   The raw String from tcp stream
    ///
    /// @return HttpRequest returns parse HttpRequest
    pub fn new_from(request: String) -> HttpRequest {
        let splits: Vec<&str> = if let Some(line) = request.lines().nth(0) {
            line.trim().split(" ").collect()
        } else {
            vec![]
        };

        if splits.len() != 3 || splits[0] != "GET" || !splits[2].contains("HTTP")  {
            return HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            };
        }

        HttpRequest {
            method: splits[0].to_string(),
            request_path: convert_path(splits[1].to_string()),
            protocol: splits[2].to_string(),
            status: HttpStatusCode::Nil,
        }
    }

    pub fn set_status(&mut self, status : HttpStatusCode) {
        self.status = status;
    }

    pub fn get_status(&self) -> HttpStatusCode {
        self.status.clone()
    }

    pub fn get_method(&self) -> &String {
        &self.method
    }

    pub fn get_path(&self) -> &String {
        &self.request_path
    }
}


impl HttpResponse {
    /// Reads from TcpStream. Blocks until completely read
    /// @param request : &HttpRequest
    /// @param status_code : &str
    /// @param payload : String
    ///
    /// @return HttpResponse returns HttpResponse object
    pub fn new_from(request: &HttpRequest, payload: String) -> HttpResponse {
        HttpResponse {
            protocol: request.protocol.clone(),
            method: request.get_method().clone(),
            status: request.get_status().clone(),
            content_type: get_content_type(request.request_path.clone()),
            content_length: payload.len(),
            payload: payload,
        }
    }

    pub fn get_content_type(&self) -> &String {
        &self.content_type
    }

    pub fn get_content_length(&self) -> usize {
        self.content_length
    }

    pub fn get_payload(&self) -> &String {
        &self.payload
    }

    pub fn get_status(&self) -> HttpStatusCode {
        self.status.clone()
    }

    pub fn get_method(&self) -> &String {
        &self.method
    }

    pub fn get_protocol(&self) -> &String {
        &self.protocol
    }

    pub fn get_status_tag(code : HttpStatusCode) -> String {
        use self::HttpStatusCode::*;

        match code {
            OK => "OK".to_string(),
            BadHttpRequest => "Bad Http Request".to_string(),
            Forbidden => "Forbidden".to_string(),
            NotFound => "Not Found".to_string(),
            _ => "Unknown Error".to_string(),
        }
    }
}


/// Used to make the path relative to directory instead of absolute.
/// Aka removes leading forward slash
///
/// @param path : String
///
/// @return String returns path string with no leading slash
fn convert_path(path: String) -> String {
    match path.find('/') {
        Some(index) if index == 0 => {
            let slice = &path[1..];
            return slice.to_owned();
        },
        _ => return path
    }
}


/// Simply takes string segment after last period in file extension to decipher type
/// If not html, returns it as plain text.
///
/// @param path : String
///
/// @return String returns "text/html" or "text/plain"
fn get_content_type(path: String) -> String {
    let mut tokens: Vec<&str> = path.split(".").collect();
    let extension = tokens.pop().unwrap();
    if extension == "html" {
        "text/html".to_string()
    } else {
        "text/plain".to_string()
    }
}


#[cfg(test)]
mod http_tests {

    mod http_request_tests {
        use super::super::{HttpRequest, HttpStatusCode};
        use super::super::HttpStatusCode::*;

        #[test]
        fn from_good1() {
            assert_http_eq( "GET index.html HTTP/1.1", HttpRequest{
                method: "GET".to_string(),
                request_path: "index.html".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: Nil,
            });
        }

        #[test]
        fn from_good2() {
            assert_http_eq( "GET Cargo.toml HTTP/1.1", HttpRequest{
                method: "GET".to_string(),
                request_path: "Cargo.toml".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: Nil,
            });
        }

        #[test]
        fn from_good3() {
            assert_http_eq( "GET Cargo.toml HTTP/1.1\n\n\nRandom Payload", HttpRequest{
                method: "GET".to_string(),
                request_path: "Cargo.toml".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: Nil,
            });
        }

        #[test]
        fn from_good4() {
            assert_http_eq( "GET Cargo.toml HTTP/1.1\nUnfortunately does not check for abosolute perfection", HttpRequest{
                method: "GET".to_string(),
                request_path: "Cargo.toml".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: Nil,
            });
        }

        #[test]
        fn from_bad_missing_protocol() {
            assert_http_eq( "GET index.html", HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            });
        }

        #[test]
        fn from_bad_get_only() {
            assert_http_eq( "POST index.html HTTP/1.1", HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            });
        }

        #[test]
        fn from_bad_few_tokens() {
            assert_http_eq( "", HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            });
        }

        #[test]
        fn from_bad_extra_tokens() {
            assert_http_eq( "GET index.html HTTP/1.0 Extra Words", HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            });
        }


        #[test]
        fn from_bad_protocol() {
            assert_http_eq( "GET index.html PoopieProtocol", HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            });
        }


        fn assert_http_eq(raw_request : &str, expected : HttpRequest) {
            assert_eq!(HttpRequest::new_from(raw_request.to_string()), expected);
        }

    }


    mod http_response_tests {
        use super::super::{HttpRequest, HttpStatusCode, HttpResponse};
        use super::super::HttpStatusCode::*;

        #[test]
        fn from_ok() {
            assert_http_eq(&HttpRequest{
                method: "GET".to_string(),
                request_path: "index.html".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: OK,
            },

            "".to_string(),

            HttpResponse {
                method: "GET".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: OK,
                content_type : "text/html".to_string(),
                payload : "".to_string(),
                content_length : 0
            });
        }


        #[test]
        fn from_ok_with_payload_not_html() {
            assert_http_eq(&HttpRequest{
                method: "GET".to_string(),
                request_path: "Cargo.toml".to_string(),
                protocol: "HTTP/1.0".to_string(),
                status: OK,
            },

            "test".to_string(),

            HttpResponse {
                method: "GET".to_string(),
                protocol: "HTTP/1.0".to_string(),
                status: OK,
                content_type : "text/plain".to_string(),
                payload : "test".to_string(),
                content_length : 4,
            });
        }


        #[test]
        fn from_good_payload_new_protocol() {
            assert_http_eq(&HttpRequest{
                method: "GET".to_string(),
                request_path: "index.html".to_string(),
                protocol: "HTTP/1.0".to_string(),
                status: OK,
            },

            "test".to_string(),

            HttpResponse {
                method: "GET".to_string(),
                protocol: "HTTP/1.0".to_string(),
                status: OK,
                content_type : "text/html".to_string(),
                payload : "test".to_string(),
                content_length : 4
            });
        }

        #[test]
        fn from_bad() {
            assert_http_eq(

            &HttpRequest {
                protocol: "HTTP/1.1".to_string(),
                method: "1.1".to_string(),
                request_path: "".to_string(),
                status: HttpStatusCode::BadHttpRequest,
            },

            "".to_string(),

            HttpResponse {
                method: "1.1".to_string(),
                protocol: "HTTP/1.1".to_string(),
                status: BadHttpRequest,
                content_type : "text/plain".to_string(),
                payload : "".to_string(),
                content_length : 0
            });
        }


        fn assert_http_eq(req : &HttpRequest, payload : String, expected : HttpResponse) {
            assert_eq!(HttpResponse::new_from(req, payload), expected);
        }
    }
}
