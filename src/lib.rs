#[macro_use] extern crate ruru;
extern crate hyper;

pub mod server;

use ruru::VM;

#[no_mangle]
pub extern fn initialize_busy() {
    // VM::require("/Users/sloveless/.gem/ruby/2.2.5/gems/rack-2.0.1/lib/rack/builder");
    VM::require("rack/builder");
    server::init();
}
