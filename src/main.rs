extern crate hyper;
extern crate ruru;
extern crate busy;

use busy::rack_env::RackEnv;
use hyper::server::{Server, Request, Response};
use ruru::{Array, Class, Object, RString, VM};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() {
    VM::init();
    VM::require("/Users/sloveless/.gem/ruby/2.2.5/gems/rack-2.0.1/lib/rack");
    VM::require("/Users/sloveless/.gem/ruby/2.2.5/gems/rack-2.0.1/lib/rack/builder");

    let rackup_file_path = env::args().nth(1).unwrap();
    println!("The first argument is {}", rackup_file_path);

    // Read the rackup file
    let mut f = File::open(&rackup_file_path).unwrap();
    let mut s = String::new();
    let _result = f.read_to_string(&mut s);
    let ruby_rackup_file_contents = RString::new(s.as_str()).to_any_object();
    let ruby_rackup_file_path = RString::new(rackup_file_path.as_str()).to_any_object();

    let ruby_builder = Class::from_existing("Rack")
        .get_nested_class("Builder")
        .new_instance(vec![]);

    let ruby_app = ruby_builder
        .send("instance_eval", vec![ruby_rackup_file_contents, ruby_rackup_file_path]);

    // let rails_root_string = env::args().nth(1).unwrap();
    // let rails_root_path = Path::new(rails_root_string.as_str());
    // println!("The first argument is {}", rails_root_path.display());

    // let rails_env_file_path = rails_root_path.join("config/environment");
    // println!("Env file: {}", rails_env_file_path.display());

    // VM::require(rails_env_file_path.to_str().unwrap());

    // This needs to build up an `env` to match the rack spec, then call
    // Rails.application.call(env).
    Server::http("0.0.0.0:8080").unwrap().handle(move |req: Request, res: Response| {
        println!("request uri: {}", req.uri);
        let rack_env = RackEnv::from(req);

        // match ruby_app.send("call", vec!(rack_env.env.to_any_object())) {
        //     Err(ref error) => VM::raise(error.to_exception(), error.description()),
        //     Ok(_ruby_response) => {
        //         println!("yay");
        //     }
        // };
        let app_class_name = ruby_app.send("class", vec![]);
        Class::from_existing("Kernel").send("puts", vec![app_class_name]);

        Class::from_existing("Kernel").send("puts", vec![rack_env.env.to_any_object()]);
        let ruby_result = ruby_app.send("call", vec!(rack_env.env.to_any_object()))
                            .try_convert_to::<Array>();

        println!("meow");

        match ruby_result {
            Err(ref error) => VM::raise(error.to_exception(), error.description()),
            Ok(_ruby_response) => {
                println!("yay");
            }
        };

        println!("sup");

        res.send(b"Hello World!").unwrap();
    }).unwrap();
}
