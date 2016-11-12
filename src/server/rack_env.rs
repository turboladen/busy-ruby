use hyper::header::{ContentLength, Headers};
// use hyper::http::h1::{HttpWriter};
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use ruru::{Class};
// use hyper::version::HttpVersion;

use ruru::{Array, Boolean, Fixnum, Hash, Object, RString};

pub struct RackEnv {
    pub env: Hash
}

impl<'a, 'b> From<Request<'a, 'b>> for RackEnv {
    fn from(req: Request) -> Self {
        let mut env = Hash::new();

        if let RequestUri::AbsolutePath(uri) = req.uri {
            let uri = uri.as_str();
            println!("URI: {}", uri);

            env.store(RString::new("PATH_INFO"), RString::new(uri));
            env.store(RString::new("REQUEST_PATH"), RString::new(uri));
            env.store(RString::new("REQUEST_URI"), RString::new(uri));
        };

        env.store(RString::new("HTTP_HOST"), RString::new("localhost:8080"));
        env.store(RString::new("REMOTE_ADDR"), RString::new("127.0.0.1"));
        env.store(RString::new("REQUEST_METHOD"), RString::new("GET"));
        env.store(RString::new("SERVER_NAME"), RString::new("localhost"));
        env.store(RString::new("SERVER_PORT"), RString::new("8080"));

        // let http_version = format!("{}", req.version);
        // env.store(RString::new("HTTP_VERSION"), RString::new(http_version.as_str()));
        // env.store(RString::new("SERVER_PROTOCOL"), RString::new(http_version.as_str()));
        env.store(RString::new("HTTP_VERSION"), RString::new("HTTP/1.1"));
        env.store(RString::new("SERVER_PROTOCOL"), RString::new("HTTP/1.1"));
        env.store(RString::new("SERVER_SOFTWARE"), RString::new("busy bruh"));

        env.store(RString::new("rack.input"), RString::new(""));
        env.store(RString::new("rack.multiprocess"), Boolean::new(false));
        env.store(RString::new("rack.thread"), Boolean::new(true));
        env.store(RString::new("rack.url_scheme"), RString::new("http"));

        // let rack_version = Class::from_existing("Rack").const_get("VERSION");
        let mut rack_version = Array::with_capacity(2);
        rack_version.push(Fixnum::new(1));
        rack_version.push(Fixnum::new(3));
        env.store(RString::new("rack.version"), rack_version);

        RackEnv { env: env }
    }
}

pub fn rack_to_response(rack_array: Array, res: &mut Response) -> String {
    println!("in rack to response");
    // Set status
    let rack_status = rack_array.at(0).try_convert_to::<Fixnum>().unwrap().to_i64();
    let hyper_status = StatusCode::from_u16(rack_status as u16);
    // *hyper_response.status_mut() = hyper_status;

    println!("status: {}", hyper_status);
    //------------------------
    // Set headers
    let mut headers = Headers::new();

    // Set Content-Length
    let ruby_body = rack_array.at(2).try_convert_to::<RString>().unwrap();
    let ruby_body_class = ruby_body.send("class", vec![]);
    Class::from_existing("Kernel").send("puts", vec![ruby_body_class]);

    let body = ruby_body.to_string();
    headers.set(ContentLength(body.len() as u64));
    // End headers
    //------------------------

    println!("body: {}", body);
    *res.status_mut() = hyper_status;
    body
}
