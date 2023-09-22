#![allow(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32g0xx_hal::cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use hal::gpio::gpioa::PA5;
use hal::prelude::*;
use hal::rcc::{self, PllConfig};
use hal::spi::{NoMiso, NoSck, Mode, Polarity, Phase};
use hal::stm32;
use rt::entry;
use nb::block;
use smart_leds_trait::{SmartLedsWrite, RGB};
use neopixels::{ws2812_generate };

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");

    // Configure APB bus clock to 24MHz, cause ws2812 requires 4Mbps SPI
    let pll_cfg = PllConfig::with_hsi(1, 4, 2);
    let cfg = rcc::Config::pll().pll_cfg(pll_cfg);
    //let cfg = rcc::Config::hsi(rcc::Prescaler::NotDivided);
    let mut rcc = dp.RCC.freeze(cfg);

    let mut delay = cp.SYST.delay(&mut rcc);
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    /// SPI mode that can be used for this crate
    let mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let mut spi = dp
        .SPI1
        .spi((NoSck, NoMiso, gpiob.pb5), mode, 4.MHz(), &mut rcc);
    spi.half_duplex_enable(true);
    spi.half_duplex_output_enable(true);
    spi.data_size(5);

    ws2812_generate!(15);
    
    let mut neopixels = Ws2812::new();
    let mut blink = gpioa.pa5.into_push_pull_output();

    let mut cnt: usize = 0;
    neopixels.intensity( RGB { r: 0x30, g: 0x40, b: 0x80 });
    loop {
        for _ in 0..10 {
            for idx in 0..neopixels.len() {
                let color = match (cnt + idx) % 3 {
                        0 => RGB { r: 0x81, g: 0, b: 0 },
                        1 => RGB { r: 0, g: 0x81, b: 0 },
                        _ => RGB { r: 0, g: 0, b: 0x81 },
                };
                neopixels.set(idx, color);
            }
            cnt += 1;
            for pixel in &mut neopixels {
                for b in pixel {
                    block!(spi.send(b));
                }
            } 
            delay.delay(500.millis());
            blink.toggle().unwrap();
        }
        for i in 0..25 {
            let color = RGB { r: i * 10, g: 255-i*10, b: i };
            neopixels.shift_right();
            neopixels.set(0,color);
            for pixel in &mut neopixels {
                for b in pixel {
                    block!(spi.send(b));
                }
            } 
            delay.delay(100.millis());
            blink.toggle().unwrap();
        }
    }
}
