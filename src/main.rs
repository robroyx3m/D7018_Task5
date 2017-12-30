#![feature(used)]
#![feature(proc_macro)]
#![no_std]
#![allow(dead_code)]

extern crate blue_pill;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::Timer;
use blue_pill::led::{self, LED};
use blue_pill::prelude::*;
use blue_pill::time::Hertz;
use rtfm::{app, Threshold};
use cortex_m::peripheral::SystClkSource;

app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
    },
}

fn init(p: init::Peripherals) {
    led::init(p.GPIOC, p.RCC);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000); // 1s
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        LED.on();
    } else {
        LED.off();
    }
}
/*
fn main() {
    //asm::bkpt();

    unsafe {
        (*DWT.get()).enable_cycle_counter();
        (*DWT.get()).cyccnt.write(0);
    }
    //asm::bkpt();

    // get a handle to the *host* standard output
    let mut stdout = hio::hstdout().unwrap();

    let wordarr: u32 = 0;
    let bytearr: u32 = 0;
    decode(wordarr, bytearr);
    writeln!(stdout, "Decoded string: ");

    unsafe {
        for x in PLAIN.iter() {
            write!(stdout, "{}", *x as char);
        }
    }
    write!(stdout, "\n");
    // asm::bkpt();
    //  asm::bkpt();
    unsafe {
        write!(stdout, "{}", (*DWT.get()).cyccnt.read());
    }
}

fn decode(mut wordarr: u32, mut bytearr: u32) -> u32 {
    let x = !codgen();
    let mut r: u32;
    let y: u32;
    let m: u32;

    if ABC[wordarr as usize] == 0 {
        unsafe {
            PLAIN[bytearr as usize] = 0;
        }
        r = x;
    } else {
        wordarr += 1;
        bytearr += 1;
        y = decode(wordarr, bytearr);
        wordarr -= 1;
        bytearr -= 1;
        m = x.wrapping_sub(y).wrapping_sub(ABC[wordarr as usize]);
        let mut temp = m >> 13; //To aqquire [20..13]
        temp = temp & 0xff;
        unsafe {
            PLAIN[bytearr as usize] = temp as u8;
        }
        r = !codgen() + 1;
        r = x.wrapping_add(y)
            .wrapping_add(m)
            .wrapping_add(r)
            .wrapping_add(5);
    }
    r
}

fn codgen() -> u32 {
    let n: i32;
    let x: u32;
    let y: u32;
    let mut local_seed: u32;

    unsafe {
        local_seed = SEED;
    }

    n = local_seed.count_zeros() as i32;

    x = local_seed.rotate_left(30);

    // y = right arithmeic shift seed 6 bits
    y = ((local_seed as i32) >> 6) as u32;

    local_seed = x ^ y;

    unsafe {
        SEED = local_seed ^ (n as u32);
        SEED ^ 0x464b713e // Return seed XOR mask
    }
}
*/