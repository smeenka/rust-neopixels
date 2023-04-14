# rust-neopixels

Generic neopixel library for Rust and Embassy

This library does not have dependencies towards the spi or bitbang libraries.

Instead, this library provides an bitbuffer, which can be send via Spi and DMA towards the neopixel leds.

Implemented will be:

- An 1 bit per byte iterator, to be used with a half-duplex Spi driver in 5bits mode, on 4 Hhz, with a system clock of 16 Mhz or multiples. Each byte will contain the patters for a one bit, so per neopixel bit 5 bits

Note that this implementation can run on 16 Mhz or higher.

The chosen implementation does consume more memory (8 bytes per pixel), bug the gain with this approach is that the library
can be used very easyily in an asycn evironment, without worrying about timing issues.

Binarys can even build without the --release flag, as the timing for the neopixels is not hardware dependend.
