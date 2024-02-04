use core::fmt::{Error, Write};

pub struct Uart {
    base_address: usize
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for byte in s.bytes() {
            self.write(byte);
        }
        
        return Ok(());
    }
}

impl Uart {
    const GLOBAL_CLOCK_RATE: u32 = 22_729_000;
    const BAUD_RATE: u32 = 2400;

    pub fn new(base_address: usize) -> Uart {
        return Uart {
            base_address
        };
    }

    pub fn init(&mut self) {
        let ptr: *mut u8 = self.base_address as *mut u8;

        unsafe {
            // Set word length by setting LCR bits 0 and 1 to 1
            let lcr = (1 << 0) | (1 << 1); // 8 bits
            ptr.add(3).write_volatile(lcr);

            // Enable FIFO
            ptr.add(2).write_volatile(1 << 0);

            // Enable reciever buffer interrupts
            ptr.add(1).write_volatile(1 << 0);

            // Set baud rate
            let divisor: u32 = Self::GLOBAL_CLOCK_RATE.div_ceil(16 * Self::BAUD_RATE);
            let divisor_lsb: u32 = divisor & 0xFF;
            let divisor_msb: u32 = divisor >> 8;

            // Open divisor latch by setting LCR bit 7 to 1
            ptr.add(3).write_volatile(lcr | (1 << 7));

            // Write divisor lsb and msb
            ptr.add(0).write_volatile(divisor_lsb as u8);
            ptr.add(1).write_volatile(divisor_msb as u8);

            // Close divisor latch by setting LCR bit 7 to 0
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn read(&self) -> Option<u8> {
        let ptr: *const u8 = self.base_address as *const u8;
        unsafe {
            return if ptr.add(5).read_volatile() & (1 << 0) != 0 {
                Some(ptr.add(0).read_volatile())
            }
            else {
                None
            };
        }
    }

    pub fn write(&self, byte: u8) {
        let ptr: *mut u8 = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(byte);
        }
    }
}
