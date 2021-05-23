#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use testing::{serial_print, serial_println};

use cmos_rtc::{ReadRTC, Time};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_cmos() {
    let mut cmos = ReadRTC::new(0x00, 0x00);
    let time: Time = cmos.read();

    serial_println!("second: {}", time.second);
    serial_println!("minute: {}", time.minute);
    serial_println!("hour: {}", time.hour);
    serial_println!("day: {}", time.day);
    serial_println!("month: {}", time.month);
    serial_println!("year: {}", time.year);

    assert_eq!(time.century, 0);
}
