#![no_std]
#![no_main]
use defmt::*;

use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    // let p = embassy_nrf::init(Default::default());

    // let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    // unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
