use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// Using lazy_static to ensure the init method 
// is called EXACTLY once on its first use
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // COM1 <=> IO Port 0x3F8
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// Abstractions to make the serial port easy to use!

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// Prints to the host through serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

// Prints to the host through serial interface, now with newlines!
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}