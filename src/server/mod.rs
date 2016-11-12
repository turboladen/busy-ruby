mod rack_env;

use self::rack_env::RackEnv;
use self::rack_env::rack_to_response;
use hyper::server::Server as HyperServer;
use hyper::server::{Request, Response};
use ruru::{AnyObject, Array, Class, NilClass, Object, RString};
use std::error::Error;
use std::io::Write;

class!(Server);

methods!(
    Server,
    _itself,

    fn rackup(file_path: RString) -> NilClass {
        let ruby_file_path = file_path.unwrap();
        Class::from_existing("Kernel").send("puts", vec![RString::new("sup").to_any_object()]);

        let app_and_options = Class::from_existing("Rack").get_nested_class("Builder")
                                .send("parse_file", vec![ruby_file_path.to_any_object()])
                                .try_convert_to::<Array>()
                                .unwrap();

        Class::from_existing("Kernel").send("puts", vec![app_and_options.to_any_object()]);
        run_hyper(app_and_options.at(0));
        NilClass::new()
    }

    fn run(app: AnyObject) -> NilClass {
        run_hyper(app.unwrap());

        NilClass::new()
    }
);

fn run_hyper(ruby_app: AnyObject) {
    HyperServer::http("0.0.0.0:8080").unwrap().handle(move |req: Request, mut res: Response| {
        println!("request uri: {}", req.uri);
        let rack_env = RackEnv::from(req);

        let app_class_name = ruby_app.send("class", vec![]);
        ruby_puts(app_class_name);
        ruby_puts(rack_env.env.to_any_object());

        let rack_response = ruby_app.send("call", vec!(rack_env.env.to_any_object()))
                                .try_convert_to::<Array>()
                                .unwrap();

        let ruby_body = rack_to_response(rack_response, &mut res);
        let body = ruby_body.as_bytes();

        let mut stream = res.start().unwrap();
        stream.write_all(body).unwrap();
        // res;
    }).unwrap();
}

fn ruby_puts(object: AnyObject) {
    Class::from_existing("Kernel").send("puts", vec![object]);
}

pub extern fn init() {
    Class::from_existing("Busy").define(|busy| {
        busy.define_nested_class("Server", None).define(|itself| {
            itself.def_self("rackup", rackup);
            itself.def_self("run", run);
        });
    });
}
