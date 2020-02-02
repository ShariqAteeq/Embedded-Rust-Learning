#![no_main]
#![no_std]
#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};
#[entry]
fn main() -> ! {
let gpioe = aux7::init().1;
// Turn on the North LED
gpioe.bsrr.write(|w| w.bs9().set_bit());
// Turn on the East LED
gpioe.bsrr.write(|w| w.bs11().set_bit());
// Turn off the North LED
gpioe.bsrr.write(|w| w.br9().set_bit());
// Turn off the East LED
gpioe.bsrr.write(|w| w.br11().set_bit());
loop {}
}