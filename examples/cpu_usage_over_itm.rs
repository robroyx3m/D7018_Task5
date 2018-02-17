// IMPORTANT always include this feature gate
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm; // IMPORTANT always do this rename
                                    //extern crate stm32f40x;
extern crate f4;

use rtfm::{app, Resource, Threshold};
use cortex_m::peripheral::SystClkSource;
use cortex_m::{iprint, iprintln};

const CLOCK_SPEED: u32 = 16_000_000;

app!{
    device: f4::stm32f40x,

    resources:{
        static SLEEP_TIME: u32 = 0;
    },

    idle:{
        resources: [DWT, SLEEP_TIME],
    },

    tasks:{
        SYS_TICK: {
            path: send_data,
            priority: 1,
            resources: [SLEEP_TIME, ITM],
        },
    },
}

fn init(_p: ::init::Peripherals, _r: ::init::Resources) {
    // Sys tick
    _p.SYST.set_clock_source(SystClkSource::Core);
    _p.SYST.set_reload(64_000_000); // Once every 1 s
    _p.SYST.enable_interrupt();
    _p.SYST.enable_counter();

    // Cycle counter
    unsafe {
        _p.DWT.cyccnt.write(0);
        _p.DWT.sleepcnt.write(0);
    };
    _p.DWT.enable_cycle_counter();
}

fn idle(t: &mut Threshold, mut r: ::idle::Resources) -> ! {
    loop {
        // Sleep
        let before: u32 = r.DWT.cyccnt.read();
        rtfm::wfi();
        let after: u32 = r.DWT.cyccnt.read();

        let elapsed: u32 = after.wrapping_sub(before);

        // Accumulate sleep time
        r.SLEEP_TIME.claim_mut(t, |sleep_const, _| {
            **sleep_const = (**sleep_const).wrapping_add(elapsed);
        });
    }
}

fn send_data(t: &mut Threshold, r: SYS_TICK::Resources) {
    let mut sleep_time: f32 = 0.0;

    /*The value for the sleep_time is very strange 
    as I get approx 13.4 million clock cycles sleep time, I would 
    suspect this to be more like 60 million running the mcu at 64Mhz.
    */
    r.SLEEP_TIME.claim(t, |sleep_const, _| {
        sleep_time = **sleep_const as f32;
    });

    //CPU_USAGE percent

    let cpu_usage: f32 = ((CLOCK_SPEED as f32 - sleep_time) / (CLOCK_SPEED as f32)) * 100.0;

    iprintln!(&r.ITM.stim[0], "CPU usage: {} %", cpu_usage);
    // Reset sleep time back to zero
    r.SLEEP_TIME.claim_mut(t, |sleep_const, _| {
        **sleep_const = 0;
    });
}
