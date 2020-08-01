
pub struct UART([u8; 8]);

impl UART {
    pub fn can_transmit(&self) -> bool {
        (self.0[5] & 0x20) != 0
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.can_transmit() {}
        self.0[0] = byte;
    }
}

use core::fmt;
impl fmt::Write for UART {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::printfn(format_args!($($arg)*)));
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
pub fn printfn(args: fmt::Arguments) {
    let uart = unsafe { (0x10000000 as *mut UART).as_mut().unwrap() };
    use core::fmt::Write;
    uart.write_fmt(args).unwrap();
}
