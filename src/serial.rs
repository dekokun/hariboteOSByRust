use uart_16550::SerialPort;
// use spin::Mutex;


pub fn print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    let mut serial_port = SerialPort::new(0x3F8);
    serial_port.init();
    unsafe {
        static mut SERIAL1: *mut SerialPort = 0 as *mut SerialPort;
        SERIAL1 = &mut serial_port;
        (*SERIAL1).write_fmt(args).expect("Printing to serial failed");
    }
}

/// Prints to the host through the serial interface.
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}
