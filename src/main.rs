#![no_std]

mod serial;

use crate::serial::Serial;

extern "C" {
    pub fn usleep(ns: i32);
}

#[no_mangle]
pub extern "C" fn main() {
    Serial::init(56700);

    unsafe {
        usleep(1500000);
    }

    Serial::report("Hello from Rust report()\n\r");
    Serial::report("Another report()\n\r");
    Serial::report("Another test for funsies\n\r");

    unsafe {
        usleep(2000);
    }
}