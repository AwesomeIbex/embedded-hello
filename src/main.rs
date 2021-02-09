#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f4xx_hal::{delay::Delay, prelude::*}; // STM32F1 specific functions

#[allow(unused_imports)]
use panic_halt;
use stm32f4xx_hal::stm32::Peripherals;
use stm32f4xx_hal::gpio::Speed; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();

    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);
    loop {
        led.set_high().unwrap();
        delay.delay_ms(500_u32);
        led.set_low().unwrap();
        delay.delay_ms(500_u32);
    }
}
