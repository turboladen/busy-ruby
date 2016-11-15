#[macro_use] extern crate ruru;
extern crate hyper;

pub mod server;
pub mod ruby_utils;

use ruru::VM;

#[no_mangle]
pub extern fn initialize_busy() {
    VM::require("rack/builder");
    server::init();
}
