#![no_main]
#![no_std]

use foo as _;
use rtt_target::rtt_init_defmt; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_defmt!(rtt_target::ChannelMode::BlockIfFull);
    for i in 0..1000 {
        defmt::println!("{}", i);
    }
    foo::exit()
}
