#![no_std]

mod serial;
mod time;

use crate::serial::Serial;
use crate::time::Time;

#[no_mangle]
pub extern "C" fn main() {
    Serial::init(56700);

    Time::usleep(1500000);

    Serial::report("Hello from Rust report()\n\r");
    Serial::report("Another report()\n\r");
    Serial::report("Another test fun for funsies\n\r");

    Time::usleep(2000);
}