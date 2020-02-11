#![no_std]
#![no_main]

use stm32f4 as hal;
use cortex_m_rt as rt;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub fn init_itm() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut itm = init_itm();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    loop {
        iprintln!(&mut itm.stim[0], "Hello, world!");
    }
}
