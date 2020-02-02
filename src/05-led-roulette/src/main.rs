#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 80_u16;

    loop {
        for i in 0..8{
        leds[i].on();
        delay.delay_ms(half_period);
       
        }
        for j in 0..8{
        leds[j].off();
        delay.delay_ms(half_period);
        }
    }
}
