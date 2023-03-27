#![no_std]
#![no_main]

extern crate panic_halt;

use core::convert::Infallible;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::ToggleableOutputPin;
use stm32f0xx_hal::delay::Delay;
use stm32f0xx_hal::prelude::*;
use stm32f0xx_hal::stm32;

struct Led<'a> {
    pin: &'a mut dyn ToggleableOutputPin<Error = Infallible>,
}

impl Led<'_> {
    fn new(pin: &mut dyn ToggleableOutputPin<Error = Infallible>) -> Led {
        Led { pin }
    }

    fn toggle(&mut self) -> Result<(), Infallible> {
        self.pin.toggle()
    }
}

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    let mut peripherals = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    cortex_m::interrupt::free(|cs| {
        let mut rcc = peripherals
            .RCC
            .configure()
            .sysclk(8.mhz())
            .freeze(&mut peripherals.FLASH);

        let gpioc = peripherals.GPIOC.split(&mut rcc);

        let mut delay = Delay::new(cp.SYST, &rcc);

        let orange = gpioc.pc8.into_push_pull_output(cs);
        let green = gpioc.pc9.into_push_pull_output(cs);
        let red = gpioc.pc6.into_push_pull_output(cs);
        let blue = gpioc.pc7.into_push_pull_output(cs);

        let mut orange = orange.downgrade();
        let mut green = green.downgrade();
        let mut red = red.downgrade();
        let mut blue = blue.downgrade();

        let mut leds: [Led; 4] = [
            Led::new(&mut orange),
            Led::new(&mut green),
            Led::new(&mut red),
            Led::new(&mut blue),
        ];

        let mut i = 0;

        loop {
            leds[i % 4].toggle().unwrap();
            i += 1;
            delay.delay_ms(20_u16);
        }
    })
}
