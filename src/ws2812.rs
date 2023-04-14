//! # Use ws2812 leds via spi
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! The spi peripheral should run at 4MHz. Each neopixel bit is 5 SPI bits.
//! In this way the neopixel timing is exact 800 Khrz, and the timing for the low and high bits
//! is within the specification.
//!            specification (ns)      this variant (ns)      lib.rs variant (ns)
//! low bit:   800/450                 725/500                 999/333
//! high bit   400/850                 500/750                 333/999
//! frequency  800 Khz                 800Khz                  750 Khz
//!
//! The max deviation for the timing here is 100 ns, where the max deviation of the timing in lib.rs
//! is 199 us.
//!
//! Due to the use of half duplex, the tx fifo of the spi is optimal used , so 6 neopixel bits (== 30 spi bits)
//! can be stored in the fifo.
//!
//! During the calculation of the next pixel one can see that the bit pattern is stretched (at 16 mhz cpu clock),
//! so calculation takes longer than the six neopixel bits in the fifo (longer than 6 * 1.25 us = 7.5 us).
//!
//! Note that the 5 bit pattern always has a zero at the beginning and the end. In this way stretching  will always
//! happen during a zero phase. The zero phase is less time sensitive than the one phase.
//!  
//! This variant can run on a cpu clock of 16 Mhz and multiples (32, 64)
//!
//! Note that this halfduplex variant can only run with a feature branch of the hal.
//! where the fullduplex spi can be switched to half duplex output mode and 5 bits wide operation
//! An example how to use this variant can be found in the examples of this feature branch see
//! https://github.com/smeenka/rust-examples/blob/master/nucleo-G070/examples/neopixel5bits.rs
//!
//
// Timings for ws2812 from https://cpldcpu.files.wordpress.com/2014/01/ws2812_timing_table.png

#[allow(unused_macros)]
#[macro_export]
macro_rules! ws2812_generate {
    ($nr_pixels:literal) => {
        use embassy_stm32::spi::U5;
        use neopixels::{Neopixels, RGB};
        const BITS_PER_PIXEL: usize = 24;
        const TOTAL_BITS: usize = $nr_pixels * BITS_PER_PIXEL;
        const OFFSET_G: usize = 0;
        const OFFSET_R: usize = 8;
        const OFFSET_B: usize = 16;

        pub struct Ws2812 {
            intensity_r: u8,
            intensity_g: u8,
            intensity_b: u8,
            // Note that the U5 type controls the selection of 5 bits to output
            bitbuffer: [U5; TOTAL_BITS],
        }
        impl Ws2812 {
            pub fn new() -> Ws2812 {
                Ws2812 {
                    intensity_r: 0x50 as u8,
                    intensity_g: 0x60 as u8,
                    intensity_b: 0x80 as u8,
                    bitbuffer: [U5(0); TOTAL_BITS],
                }
            }
            pub fn len(&self) -> usize {
                return $nr_pixels as usize;
            }
            // transform one color byte into an array of 8 byte. Each byte in the array does represent 1 neopixel bit pattern
            fn render_color(&mut self, pixel_idx: usize, offset: usize, color: usize) {
                let mut bits = color as usize;
                let mut idx = pixel_idx * BITS_PER_PIXEL + offset;

                // render one bit in one spi byte. High time first, then the low time
                // clock should be 4 Mhz, 5 bits, each bit is 0.25 us.
                // a one bit is send as a pulse of 0.75 high -- 0.50 low
                // a zero bit is send as a pulse of 0.50 high -- 0.75 low
                // clock frequency for the neopixel is exact 800 khz
                // note that the mosi output should have a resistor to ground of 10k,
                // to assure that between the bursts the line is low
                for i in 0..8 {
                    if idx >= TOTAL_BITS {
                        return;
                    }
                    let pattern: u8 = match bits & 0x80_00 {
                        0x80_00 => 0b0000_1110,
                        _ => 0b000_1100,
                    };
                    bits = bits << 1;
                    self.bitbuffer[idx] = U5(pattern);
                    idx += 1;
                }
            }
        }
        impl Neopixels for Ws2812 {
            fn set_r(&mut self, idx: usize, r: u8) {
                let color = (self.intensity_r as usize * r as usize);
                self.render_color(idx, OFFSET_R, color);
            }
            fn set_g(&mut self, idx: usize, g: u8) {
                let color = (self.intensity_g as usize * g as usize);
                self.render_color(idx, OFFSET_G, color);
            }
            fn set_b(&mut self, idx: usize, b: u8) {
                let color = (self.intensity_b as usize * b as usize);
                self.render_color(idx, OFFSET_B, color);
            }
            fn set_w(&mut self, idx: usize, w: u8) {}
            fn set(&mut self, idx: usize, rgb: RGB) {
                self.set_g(idx, rgb.g);
                self.set_r(idx, rgb.r);
                self.set_b(idx, rgb.b);
            }
            fn shift_left(&mut self) {}
            fn shift_right(&mut self) {}
            fn rotate_left(&mut self) {}
            fn rotate_right(&mut self) {}

            fn intensity_r(&mut self, r: u8) {
                self.intensity_r = r;
            }
            fn intensity_g(&mut self, g: u8) {
                self.intensity_g = g;
            }
            fn intensity_b(&mut self, b: u8) {
                self.intensity_b = b;
            }
            fn intensity_w(&mut self, w: u8) {}
            fn intensity(&mut self, rgb: RGB) {
                self.intensity_r = rgb.r;
                self.intensity_g = rgb.g;
                self.intensity_b = rgb.b;
            }
        }
    };
}
