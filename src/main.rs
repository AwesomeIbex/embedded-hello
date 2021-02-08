#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f4xx_hal::{delay::Delay, prelude::*}; // STM32F1 specific functions

#[allow(unused_imports)]
use panic_halt;
use stm32f4xx_hal::stm32::Peripherals; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, clocks);
    loop {
        led.set_high().ok();
        delay.delay_ms(1_000_u16);
        led.set_low().ok();
        delay.delay_ms(1_000_u16);
    }
}
