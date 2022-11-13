use core::fmt;
use crate::vga;
use crate::serial;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    // TODO: console detection
    vga::WRITER.lock().write_fmt(args).expect("Printing to VGA failed");
    serial::SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}
