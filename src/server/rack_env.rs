use hyper::header::{ContentLength};
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use ruru::{AnyObject, Array, Boolean, Class, Fixnum, Hash, Object, RString};
use self::super::super::ruby_utils;

pub struct RackEnv {
    pub env: Hash
}

impl<'a, 'b> From<Request<'a, 'b>> for RackEnv {
    fn from(req: Request) -> Self {
        let mut env = Hash::new();
        env.store(RString::new("SERVER_SOFTWARE"), RString::new("busy bruh"));

        // TODO: handle relative paths
        if let RequestUri::AbsolutePath(uri) = req.uri {
            let uri = uri.as_str();
            println!("URI: {}", uri);

            // TODO: Not sure if these should all be the same
            env.store(RString::new("PATH_INFO"), RString::new(uri));
            env.store(RString::new("REQUEST_PATH"), RString::new(uri));
            env.store(RString::new("REQUEST_URI"), RString::new(uri));
        };

        let remote_addr = format!("{}", req.remote_addr.ip());
        env.store(RString::new("REMOTE_ADDR"), RString::new(remote_addr.as_str()));

        let request_method = format!("{}", req.method);
        env.store(RString::new("REQUEST_METHOD"), RString::new(request_method.as_str()));

        let http_version = format!("{}", req.version);
        env.store(RString::new("HTTP_VERSION"), RString::new(http_version.as_str()));
        env.store(RString::new("SERVER_PROTOCOL"), RString::new(http_version.as_str()));

        env.store(RString::new("SERVER_NAME"), RString::new("localhost"));
        env.store(RString::new("SERVER_PORT"), RString::new("8080"));

        //---------------------------------------------------------------------
        // Headers
        for header in req.headers.iter() {
            let name = format!("HTTP_{}", header.name().to_uppercase());
            let value = header.value_string();
            env.store(RString::new(name.as_str()), RString::new(value.as_str()));
        }

        //---------------------------------------------------------------------
        // Rack-specific
        env.store(RString::new("rack.input"), RString::new(""));
        env.store(RString::new("rack.multiprocess"), Boolean::new(false));
        env.store(RString::new("rack.thread"), Boolean::new(true));
        env.store(RString::new("rack.url_scheme"), RString::new("http"));

        let rack_version = Class::from_existing("Rack").const_get("VERSION");
        env.store(RString::new("rack.version"), rack_version);

        RackEnv { env: env }
    }
}

pub fn rack_to_response(rack_array: Array, res: &mut Response) -> String {
    // Set status
    let hyper_status = build_status(rack_array.at(0));
    *res.status_mut() = hyper_status;
    println!("status: {}", hyper_status);

    //------------------------
    // Read body
    let body = build_body(rack_array.at(2));

    //------------------------
    // Set headers
    res.headers_mut().set(ContentLength(body.len() as u64));
    ruby_utils::ruby_puts(RString::new("Rack response headers: ").to_any_object());
    ruby_utils::ruby_puts(rack_array.at(1));
    // End headers
    //------------------------

    println!("body: {}", body);
    body
}

fn build_status(rack_response_code: AnyObject) -> StatusCode {
    let rack_status = rack_response_code.try_convert_to::<Fixnum>().unwrap().to_i64();

    StatusCode::from_u16(rack_status as u16)
}

// This is a Rack::BodyProxy when used with Rails
fn build_body(rack_response_body: AnyObject) -> String {
    let ruby_body = Class::from_existing("Busy").send("extract_rack_proxy", vec![rack_response_body]);

    ruby_body.try_convert_to::<RString>().unwrap().to_string()
}
