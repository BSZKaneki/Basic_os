use x86_64::instructions::port::Port;
use spin::Mutex;

// Commands
const PIC_INIT: u8 = 0x11;
const PIC_MODE_8086: u8 = 0x01;
const PIC_EOI: u8 = 0x20;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = 40;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub struct ChainedPics {
    master_cmd: Port<u8>,
    master_data: Port<u8>,
    slave_cmd: Port<u8>,
    slave_data: Port<u8>,
    
    master_offset: u8,
    slave_offset: u8,
}

impl ChainedPics {
    /// Create a new interface for the chained PICs.
    /// 
    /// This constructor is marked unsafe because passing incorrect offsets
    /// can map interrupts to vectors reserved by the CPU, causing undefined behavior.
    pub const unsafe fn new(master_offset: u8, slave_offset: u8) -> Self {
        Self {
            master_cmd: Port::new(0x20),
            master_data: Port::new(0x21),
            slave_cmd: Port::new(0xA0),
            slave_data: Port::new(0xA1),
            master_offset,
            slave_offset,
        }
    }

    /// Check if a particular interrupt ID originates from this PIC configuration.
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        (self.master_offset <= interrupt_id && interrupt_id < self.master_offset + 8) ||
        (self.slave_offset <= interrupt_id && interrupt_id < self.slave_offset + 8)
    }

    /// Safely initialize the PIC.
    /// This function is unsafe because writing to incorrect CPU ports
    /// can cause immediate hardware faults or crash the CPU.
    pub unsafe fn initialize(&mut self) {
        unsafe {
            // 1. Save masks
            let mask1 = self.master_data.read();
            let mask2 = self.slave_data.read();

            // 2. Start initialization sequence (ICW1)
            self.master_cmd.write(PIC_INIT);
            self.io_wait();
            self.slave_cmd.write(PIC_INIT);
            self.io_wait();

            // 3. Set offsets (ICW2)
            self.master_data.write(self.master_offset);
            self.io_wait();
            self.slave_data.write(self.slave_offset);
            self.io_wait();

            // 4. Configure cascading (ICW3)
            self.master_data.write(4);
            self.io_wait();
            self.slave_data.write(2);
            self.io_wait();

            // 5. Set 8086 mode (ICW4)
            self.master_data.write(PIC_MODE_8086);
            self.io_wait();
            self.slave_data.write(PIC_MODE_8086);
            self.io_wait();

            // 6. Restore masks
            self.master_data.write(mask1);
            self.slave_data.write(mask2);
        }
    }

    /// Notify PIC of EOI.
    /// This function is unsafe because sending an EOI out of order or 
    /// for the wrong interrupt will lock up hardware interrupts.
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            unsafe {
                if interrupt_id >= self.slave_offset && interrupt_id < self.slave_offset + 8 {
                    self.slave_cmd.write(PIC_EOI);
                }
                self.master_cmd.write(PIC_EOI);
            }
        }
    }

    /// Tiny I/O delay helper.
    unsafe fn io_wait(&self) {
        unsafe {
            let mut port = Port::new(0x80);
            port.write(0u8);
        }
    }
}