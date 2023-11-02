extern crate console_error_panic_hook;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    orame::start()
}
