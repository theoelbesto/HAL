use avr_device::atmega328p;

#[derive(Debug, Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy)]
pub enum PinState {
    Low,
    High,
}

pub struct GpioPin<'a> {
    port: &'a atmega328p::PORTB,
    ddr: &'a atmega328p::DDRB, 
    pin: u8,  
}

impl<'a> GpioPin<'a> {
    pub fn new(port: &'a atmega328p::PORTB, ddr: &'a atmega328p::DDRB, pin: u8) -> Self {
        GpioPin { port, ddr, pin }
    }
    pub fn set_mode(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => (*self.ddr).ddrb.modify(|r| r & !(1 << self.pin)),  // Clear bit for input
                PinMode::Output => (*self.ddr).ddrb.modify(|r| r | (1 << self.pin)), // Set bit for output
            }
        }
    }
    pub fn write(&self, state: PinState) {
        unsafe {
            match state {
                PinState::High => (*self.port).portb.modify(|r| r | (1 << self.pin)), // Set bit to HIGH
                PinState::Low => (*self.port).portb.modify(|r| r & !(1 << self.pin)), // Clear bit to LOW
            }
        }
    }
    pub fn read(&self) -> PinState {
        unsafe {
            if (*self.port).pinb.read() & (1 << self.pin) != 0 {
                PinState::High
            } else {
                PinState::Low
            }
        }
    }
}
