use ruru::{AnyObject, Class, Fixnum, NilClass, Object, RString};

class!(RackInput);

methods!(
    RackInput,
    _itself,

    // Returns a (Ruby) String or nil (on EOF).
    fn rack_input_gets() -> AnyObject {
        println!("gets called");
        RString::new("").to_any_object()
    }

    // Behaves like Ruby's IO#read([length, [buffer]]).
    //
    // If +length+ given:
    //     * it must be non-negative or nil
    //     * `buffer` must be a `String` and must _not_ be `nil`.
    //
    // If `length` is not nil:
    //     * the function reads at most `length` bytes from the input stream.
    //     * the function returns `nil` at EOF.
    //
    // If `length` is nil/not given:
    //     * the function reads all data until EOF.
    //     * the function returns "" at EOF.
    //
    // If `buffer` is given, the read data is placed into that variable instead
    // of creating a new `String`.
    fn rack_input_read(_length: Fixnum, _buffer: RString) -> AnyObject {
        println!("read called");
        NilClass::new().to_any_object()
    }

    // Must be called without arguments and only yields `String`s.
    fn rack_input_each() -> NilClass {
        println!("each called");
        NilClass::new()
    }

    // Rewinds the input `String` back to the beginning. It cannot be a pipe or
    // a socket.
    fn rack_input_rewind() -> NilClass {
        println!("rewind called");
        NilClass::new()
    }

    // Must never be called on the input stream. (wat??)
    fn rack_input_close() -> NilClass {
        println!("close called");
        NilClass::new()
    }
);

pub extern fn init() {
    Class::from_existing("Busy").define(|busy| {
        busy.define_nested_class("RackInput", None).define(|itself| {
            itself.def("gets", rack_input_gets);
            itself.def("read", rack_input_read);
            itself.def("each", rack_input_each);
            itself.def("rewind", rack_input_rewind);
            itself.def("close", rack_input_close);
        });
    });
}
