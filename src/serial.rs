
extern "C" {
    pub fn serial_init(baud: i32);
    pub fn serial_putc(ns: i32);
    pub fn serial_flush();
    pub fn report(str: &str);
}

/// Responsible for handling all serial communication
pub struct Serial { }

impl Serial {
    pub fn init(baud: i32) {
        unsafe {
            serial_init(baud);
        }
    }

    pub fn putc(c: i32) {
        unsafe {
            serial_putc(c);
        }
    }

    pub fn flush() {
        unsafe {
            serial_flush();
        }
    }

    pub fn report(s: &str) {
        unsafe {
            report(s);
        }
    }
}