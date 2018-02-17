#![deny(unsafe_code)]
#![deny(warnings)]
// IMPORTANT always include this feature gate
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm; // IMPORTANT always do this rename
extern crate f4;

use rtfm::app;

app!{
    device: f4::stm32f40x,

    resources: {
        static SLEEP_TIME: u32 = 0;
    },

    idle:{
        resources: [DWT, SLEEP_TIME],
    },

    tasks: {
        SEND: [ITM, SLEEP_TIME],
    },
}

fn init(p: init::Peripherals) {}

fn idle() -> ! {
    loop {
        rtfm::atomic(|thr| {
            let dwt = DWT.access(prio, thr);
            let sleep_time = SLEEP_TIME.access(prio, thr);

            // Sleep
            let before = dwt.cyccnt.read();
            rtfm::wfi();
            let after = dwt.cyccnt.read();

            let elapsed = after.wrapping_sub(before);

            // Accumulate sleep time
            sleep_time.set(sleep_time.get() + elapsed);
        });
    }
}

fn send_data(t: &mut Threshold, r: SEND::Resources) {
    //Just nop so that something will trigger the cpu
    cortex_m::asm::nop();

    let period = 64000000;
    //CPU_USAGE percent
    iprintln!(
        "CPU usage: {}",
        (period - sleep_time.get()) / (period * 100)
    );
    // Reset sleep time back to zero
    sleep_time.set(0);
}
