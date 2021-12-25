//! # Use ws2812 leds via spi
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! Needs a type implementing the `spi::FullDuplex` trait.
//!
//! The spi peripheral should run at 2MHz to 3.8 MHz

// Timings for ws2812 from https://cpldcpu.files.wordpress.com/2014/01/ws2812_timing_table.png
// Timings for sk6812 from https://cpldcpu.wordpress.com/2016/03/09/the-sk6812-another-intelligent-rgb-led/

#![no_std]

pub mod ws2812;

 
 pub trait Neopixels<T:?Sized> {

     fn set_r(&mut self, idx:usize, r:u8);
     fn set_g(&mut self, idx:usize, g:u8);
     fn set_b(&mut self, idx:usize, b:u8);
     fn set_w(&mut self, idx:usize, w:u8);
     fn set(&mut self, idx:usize, rgb:T);

     fn get(&self, idx:usize)-> T;

     fn shift_left(&mut self);
     fn shift_right(&mut self);
     fn rotate_left(&mut self);
     fn rotate_right(&mut self);

     fn intensity_r(&mut self, g:u8);
     fn intensity_g(&mut self, g:u8);
     fn intensity_b(&mut self, b:u8);
     fn intensity_w(&mut self, w:u8);
     fn intensity(&mut self, color:T);  
    

 }

