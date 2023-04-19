#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::word::U5;
use embassy_stm32::dma::NoDma;
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use neopixels::ws2812::Ws2812;
use neopixels::RGB;
use {defmt_rtt as _, panic_probe as _};

type NeoBufferType = [U5; 4 * 24];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Start test using spi as neopixel driver");

    let mut spi = Spi::new_txonly_nosck(
        p.SPI1,
        p.PB5,
        p.DMA1_CH3,
        NoDma,
        Hertz(4_000_000),
        Config::default(),
    );
    let mut buffer = [U5(0); 4 * 24];
    let mut neopixels = Ws2812::new(&mut buffer, 24);

    for int in 0..10 {
        info!("Loop with intensity {}", int * 20);
        let mut cnt: usize = 0;
        for _i in 0..10 {
            for idx in 0..neopixels.len() {
                let color = match (cnt + idx) % 3 {
                    0 => RGB {
                        r: 0xf0,
                        g: 0,
                        b: 0,
                    },
                    1 => RGB {
                        r: 0,
                        g: 0xf0,
                        b: 0,
                    },
                    _ => RGB {
                        r: 0,
                        g: 0,
                        b: 0xf0,
                    },
                };
                neopixels.set(idx, color);
            }
            cnt += 1;
            // start sending the neopixel bit patters over spi to the neopixel string
            spi.write(neopixels.bitbuffer).await.ok();
            Timer::after(Duration::from_millis(100)).await;
        }
        let newInt = 255 - (int as u8) * 20;
        neopixels.intensity(RGB {
            r: newInt,
            g: newInt,
            b: newInt,
        });

        Timer::after(Duration::from_millis(500)).await;
    }
}
