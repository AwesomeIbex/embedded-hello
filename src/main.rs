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

#[entry]
fn main() -> ! {
    let hertz = 48.mhz();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let gpioc = dp.GPIOC.split();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(hertz).freeze();

    let mut led = gpioc.pc13.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    let adcconfig = AdcConfig::default();
    let adc = Adc::adc1(dp.ADC1, true, adcconfig);

    let gpioa = dp.GPIOA.split();
    let joystick_x = gpioa.pa0.into_analog();
    let joystick_y = gpioa.pa1.into_analog();

    free(|cs| {
        *GADC.borrow(cs).borrow_mut() = Some(adc);
        *JOYSTICK_X.borrow(cs).borrow_mut() = Some(joystick_x);
        *JOYSTICK_Y.borrow(cs).borrow_mut() = Some(joystick_y);
    });

    let mut delay_time = 500_u32;
    loop {
        free(|cs| {
            if let (Some(ref mut adc), Some(ref mut x), Some(ref mut _y)) = (
                GADC.borrow(cs).borrow_mut().deref_mut(),
                JOYSTICK_X.borrow(cs).borrow_mut().deref_mut(),
                JOYSTICK_Y.borrow(cs).borrow_mut().deref_mut()) {
                if adc.convert(x, SampleTime::Cycles_480) > 0 as u16 {
                    delay_time = (delay_time * 2) as u32
                }
            }
        });

        led.set_high().unwrap();
        delay.delay_ms(delay_time);
        led.set_low().unwrap();
        delay.delay_ms(delay_time);

        free(|cs| {
            let gpioa = dp.GPIOA.split();
            let joystick_x = gpioa.pa0.into_analog();
            let joystick_y = gpioa.pa1.into_analog();

            JOYSTICK_X.borrow(cs).replace(Some(joystick_x));
            JOYSTICK_Y.borrow(cs).replace(Some(joystick_y));
        });
    }
}
