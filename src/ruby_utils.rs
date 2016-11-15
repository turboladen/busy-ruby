use ruru::{AnyObject, Class, Object};

pub fn ruby_puts(object: AnyObject) {
    Class::from_existing("Kernel").send("puts", vec![object]);
}
