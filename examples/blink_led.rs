/*Test how to configure an blink the onboard 
LED on the nucleo-64 board
*/

#![feature(used)] // feature to ensure symbols to be linked
#![no_std] // build without the Rust standard library


extern crate cortex_m_rtfm as rtfm;
extern crate f4;

use f4::led::{self, LED};
use rtfm::app;

// TASKS & RESOURCES
app! {
    device: f4::stm32f40x,
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    led::init(&p.GPIOA, &p.RCC);
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        LED.on();
        rtfm::wfi();
        LED.off();
    }
}