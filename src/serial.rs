pub struct SerialWrapper {
    pub serial: arduino_hal::Usart<
    arduino_hal::pac::USART0,
    arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PD0>, 
    arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PD1>
    >
}

impl SerialWrapper {
    // todo actually do the macro...
    pub fn println(&mut self, msg: &str) {
        ufmt::uwriteln!(&mut self.serial, "{}", msg).unwrap();
    }
}