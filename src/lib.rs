// Steve Operating System
// Stephen Marz
// 21 Sep 2019

#![no_std]
#![feature(panic_info_message)]

use core::arch::asm;

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
		use core::fmt::Write;
		let _ = write!(uart::Uart::new(0x1000_0000), $($args)+);
	});
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
		println!(
		         "line {}, file {}: {}",
		         p.line(),
		         p.file(),
		         info.message().unwrap()
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}
#[no_mangle]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C"
fn kmain() {
	let mut uart = uart::Uart::new(0x1000_0000);
	uart.init();

	println!("Welcome to FeOS!");

	// Read from user input and echo it back
	loop {
		if let Some(c) = uart.read() {
			match c {
				// Backspace
				8 | 127 => {
					print!("{}{}{}", 8 as char, ' ', 8 as char);
				}

				// Newline
				10 | 13 => {
					println!();
				}

				// ANSI escape sequence (starting with left bracket)
				0x1b => {
					if let Some(next_byte) = uart.read() {

						// Right bracket
						if next_byte == 0x5b {
							if let Some(ansi_byte) = uart.read() {
								match ansi_byte {
									0x41 => {
										println!("up-arrow");
									}
									0x42 => {
										println!("down-arrow");
									}
									0x43 => {
										println!("right-arrow");
									}
									0x44 => {
										println!("left-arrow");
									}
									_ => {
										println!("Unknown ANSI escape sequence: 0x1b 0x5b 0x{:x}", ansi_byte);
									}
								}
							}
						}
					}
				}

				// Anything else
				_ => {
					print!("{}", c as char);
				}
			}
		}
	}
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////

pub mod uart;
