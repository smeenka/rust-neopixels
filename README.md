# rust-neopixels
Generic neopixel library for Rust, without driver dependency

This library does not have dependencies towards the spi or bitbang libraries

Instead, this library provides an iterater with the bit patters which can be used by spi drivers or bitbang drivers.

This approach makes the library much more flexable.

Implemented will be:
* An 1 bit per byte iterator, to be used with a half-duplex Spi driver in 5bits mode, on 4 Hhz, with a system clock of 16 Mhz or multiples. Each byte will contain the patters for a one bit, so per neopixel bit 5 bits
* An 2 bit per byte iterator, to be used with the full-duplex Spi driver with default 8bits mode, on 3 Mhz, with a system clock of 48 Mhz or multiples. Each byte will contain the patters for 2 bits, so per neopixel bit 4 bits 
* an 1 bit per byte iterator, to be used by bitbang drivers. Each byte contians 1 bit. The consume speed will be on 800 Mhz
