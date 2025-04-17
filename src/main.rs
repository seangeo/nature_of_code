extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod nature_of_code;

fn main() {
    pretty_env_logger::init();
    nature_of_code::run();
}
