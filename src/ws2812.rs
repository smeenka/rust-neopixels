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
macro_rules!  ws2812_generate 
{
    ($nr_pixels:literal) => {
        use neopixels::{Neopixels};
        use smart_leds_trait::{RGB};
        
        #[derive(Clone, Copy)]
        enum CurrentPixel{
            R,G,B,W
        }

        pub struct Ws2812 {
            intensity_r:u8,
            intensity_g:u8,
            intensity_b:u8,
            idx:usize,
            pixels:[RGB<u8> ;$nr_pixels],
            currentPixel:CurrentPixel,
        }
        
        impl Ws2812 {
            pub fn new() -> Ws2812 {
                Ws2812 {
                    intensity_r:0x50,
                    intensity_g:0x60,
                    intensity_b:0x80,
                    pixels: [RGB::default(); $nr_pixels],
                    idx:0,
                    currentPixel:CurrentPixel::G,
                }
            }
            pub fn len(&self) -> usize {
                return $nr_pixels
            }
            // this the self.idx should be checked before this function
            fn render_next_pixel(&mut self) -> [u8;8] 
            {
                let mut buffer:[u8;8] = [0;8];    

                let neo = self.pixels[self.idx];
                let mut pixel:u32 = 0;
                self.currentPixel = match self.currentPixel {
                    CurrentPixel::R => {
                        pixel= (self.intensity_r as u32  * neo.r as u32); 
                        CurrentPixel::B
                    },
                    CurrentPixel::G => {
                        pixel= (self.intensity_g as u32  * neo.g as u32); 
                        CurrentPixel::R
                    },
                    CurrentPixel::B => {
                        pixel= (self.intensity_b as u32  * neo.b as u32); 
                        self.idx += 1;
                        CurrentPixel::G
                    },
                    _ => CurrentPixel::G
                };

                // render one bit in one spi byte. High time first, then the low time
                // clock should be 4 hrz, 5 bits, each bit is 0.25 us.
                // a one bit is send as a pulse of 0.75 high -- 0.50 low
                // a zero bit is send as a pulse of 0.50 high -- 0.75 low
                // clock frequency for the neopixel is exact 800 khz
                for i in 0..8 {
                    let pattern = 
                        match pixel & 0x80_00 {
                            0x80_00 => 0b1110_1110,
                            0x0  => 0b1110_1100,
                            _    => 0b1111_1111
                        }; 
                        pixel = pixel << 1;
                    buffer[i] =  pattern; 
                }
                buffer
            }

        }
        impl Neopixels<RGB<u8>> for Ws2812
        {

            fn set_r(&mut self, idx:usize, r:u8){
                if idx >= self.pixels.len() { return }
                self.pixels[idx ].r = r;
            }
            fn set_g(&mut self,idx:usize, g:u8){
                if idx >= self.pixels.len() { return }
                self.pixels[idx ].g = g;

            }
            fn set_b(&mut self,idx:usize, b:u8){
                if idx >= self.pixels.len() { return }
                self.pixels[idx].b = b;

            }
            fn set_w(&mut self,idx:usize, w:u8){

            }
            fn set(&mut self, idx:usize, rgb:RGB<u8>){
                if idx >= self.pixels.len() { return }
                self.pixels[idx ] = rgb;
            }

            fn get(&self,idx:usize) -> RGB<u8>{
                if idx >= self.pixels.len() { return RGB{r:0,g:0,b:0}}
                return self.pixels[idx]
            }
            fn shift_left(&mut self){
                for i in 1..self.len() {
                    self.pixels[i-1] = self.pixels[i]
                } 
            }
            fn shift_right(&mut self){
                for i in (0..self.len()-1).rev() {
                    self.pixels[i+1] = self.pixels[i]
                } 

            }
            fn rotate_left(&mut self){
                let p0 = self.pixels[0];
                self.shift_left();
                self.pixels[self.len()-1] = p0;
            }
            fn rotate_right(&mut self){
                let p0 = self.pixels[self.len()-1];
                self.shift_right();
                self.pixels[0] = p0;
            }
    
            fn intensity_r(&mut self, r:u8){
                self.intensity_r = r;
            }
            fn intensity_g(&mut self, g:u8){
                self.intensity_g = g;

            }
            fn intensity_b(&mut self, b:u8){
                self.intensity_b = b;

            }
            fn intensity_w(&mut self, w:u8){

            }
            fn intensity(&mut self,  rgb:RGB<u8>){
                self.intensity_r = rgb.r;
                self.intensity_g = rgb.g;
                self.intensity_b = rgb.b;
            }  
        }


        impl Iterator for Ws2812 {
            type Item = [u8;8];
            //let mut idx = 0;
            fn next(&mut self) -> Option<Self::Item> {
                if self.idx >= self.pixels.len() { 
                    self.idx = 0;
                    self.currentPixel = CurrentPixel::G;
                    return None 
                }
                Some(self.render_next_pixel())
            }
        }
    }
}
