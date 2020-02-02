#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // Send a single character
    //.bits method is uded to transfer all data
    //tdr is a register to store
   // usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

   // Send a string
   for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
}


    loop {}
}
