#![no_main]
#![no_std]

use foo as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    for i in 0..1000 {
        defmt::println!("{}", i);
    }
    foo::exit()
}
