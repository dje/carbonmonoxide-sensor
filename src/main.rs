#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate embedded_graphics;
extern crate panic_semihosting;

use ads1x1x::{channel as AdcChannel, Ads1x1x, FullScaleRange, SlaveAddr};
use cortex_m_rt::entry;

use embedded_hal::adc::OneShot;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;
use f3::{
    hal::{
        delay::Delay, flash::FlashExt, gpio::GpioExt, i2c::I2c, rcc::RccExt, stm32f30x,
        time::U32Ext,
    },
    led::Led,
};

use embedded_graphics::{
    fonts::{Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};

use nb::block;
use ssd1306::prelude::*;
use ssd1306::Builder;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut led: Led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
        .into();
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1);

    let manager = shared_bus::BusManager::<cortex_m::interrupt::Mutex<_>, _>::new(i2c);

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(manager.acquire()).into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let mut adc = Ads1x1x::new_ads1115(manager.acquire(), SlaveAddr::default());
    // need to be able to measure [0-5V]
    adc.set_full_scale_range(FullScaleRange::Within6_144V).unwrap();

    let mut chip_select = gpiob.pb5.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    chip_select.set_high().unwrap();

    loop {
        // Blink LED 0 to check that everything is actually running.
        // If the LED 0 does not blink, something went wrong.
        led.on();
        delay.delay_ms(50_u16);
        led.off();

        let value_ch0 = block!(adc.read(&mut AdcChannel::SingleA0)).unwrap();

        // make the numbers smaller for reading ease
        let value_ch0 = value_ch0 >> 5;

        let mut line0: heapless::String<heapless::consts::U32> = heapless::String::new();

        write!(line0, "CO: {}", value_ch0).unwrap();

        let text_style = TextStyleBuilder::new(Font8x16)
            .text_color(BinaryColor::On)
            .build();

        disp.clear();

        Text::new(&line0, Point::new(0, 16))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();

        disp.flush().unwrap();
    }
}
