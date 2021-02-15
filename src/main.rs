#![no_std]
#![no_main]

use core::borrow::Borrow;
use core::cell::RefCell;
use core::ops::{Deref, DerefMut};

use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
#[allow(unused_imports)]
use panic_halt;
use stm32f4xx_hal::{delay::Delay, prelude::*, stm32};
use stm32f4xx_hal::adc::Adc;
use stm32f4xx_hal::adc::config::{AdcConfig, Resolution, SampleTime};
use stm32f4xx_hal::gpio::{Analog, Speed};
use stm32f4xx_hal::gpio::gpioa::{PA0, PA1};
use stm32f4xx_hal::stm32::Peripherals;

// STM32F1 specific functions

// When a panic occurs, stop the microcontroller

static GADC: Mutex<RefCell<Option<Adc<stm32::ADC1>>>> = Mutex::new(RefCell::new(None));
static JOYSTICK_X: Mutex<RefCell<Option<PA0<Analog>>>> = Mutex::new(RefCell::new(None));
static JOYSTICK_Y: Mutex<RefCell<Option<PA1<Analog>>>> = Mutex::new(RefCell::new(None));
const MIDDLE_RANGE: u32 = 512;

#[entry]
fn main() -> ! {
    // let hertz = 48.mhz();
    //
    // let cp = cortex_m::Peripherals::take().unwrap();
    // let dp = Peripherals::take().unwrap();
    // let gpioc = dp.GPIOC.split();
    // let rcc = dp.RCC.constrain();
    // let clocks = rcc.cfgr.sysclk(hertz).freeze();
    //
    // let mut led = gpioc.pc13.into_push_pull_output();
    //
    // let mut delay = Delay::new(cp.SYST, clocks);
    //
    // let adcconfig = AdcConfig::default();
    // let adc = Adc::adc1(dp.ADC1, true, adcconfig);
    //
    // let gpioa = dp.GPIOA.split();
    // let mut joystick_x = gpioa.pa0.into_analog();
    // let mut joystick_y = gpioa.pa1.into_analog();
    //
    // free(|cs| {
    //     *GADC.borrow(cs).borrow_mut() = Some(adc);
    // });
    //
    // let mut delay_time = 500_u32;
    // loop {
    //     free(|cs| {
    //         if let Some(ref mut adc) = GADC.borrow(cs).borrow_mut().deref_mut() {
    //             let x: u32 = adc.read(&mut joystick_x).unwrap() as u32;
    //             let y: u32 = adc.read(&mut joystick_y).unwrap() as u32;
    //             if y > MIDDLE_RANGE {
    //                 delay_time = (delay_time + 50) as u32
    //             } else if y < MIDDLE_RANGE {
    //                 delay_time = (delay_time - 50) as u32
    //             }
    //             if x > MIDDLE_RANGE {
    //                 delay_time = 500_u32;
    //             } else if x < MIDDLE_RANGE {
    //                 delay_time = 1000_u32;
    //             }
    //         }
    //     });
    //
    //     led.set_high().unwrap();
    //     delay.delay_ms(delay_time);
    //     led.set_low().unwrap();
    //     delay.delay_ms(delay_time);
    loop {};
}
