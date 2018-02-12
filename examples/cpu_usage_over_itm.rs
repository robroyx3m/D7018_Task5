#![deny(unsafe_code)]
#![deny(warnings)]
// IMPORTANT always include this feature gate
#![feature(proc_macro)]
#![no_std]
 
extern crate cortex_m_rtfm as rtfm; // IMPORTANT always do this rename
extern crate f4;

use rtfm::app;

app!{
    device: f4,

    resources: {
        static CPU_USAGE: u32 = 0;
    },

    idle:{
        resources: [CPU_USAGE, ITM],
    },

    tasks: {

    },
}

fn init(p: init::Peripherals, r: init::Resources){
    p.;
}

fn idle() -> !{
    loop {rtfm::atomic(|thr| {
            let dwt = DWT.access(prio, thr);
            let sleep_time = SLEEP_TIME.access(prio, thr);

            // Sleep
            let before = dwt.cyccnt.read();
            rtfm::wfi();
            let after = dwt.cyccnt.read();

            let elapsed = after.wrapping_sub(before);

            // Accumulate sleep time
            sleep_time.set(sleep_time.get() + elapsed);

            //CPU_USAGE percent
            CPU_USAGE = (elapsed - sleep_time)/(elapsed*100);
        });
}

