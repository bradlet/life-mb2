#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
	display::blocking::Display,
	hal::{
        // prelude::*,
        timer::Timer,
    }
};
// use nanorand::{pcg64::Pcg64, Rng, SeedableRng};

mod life;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut counter = 0u64;
	let mut display = Display::new(board.display_pins);
	let mut timer = Timer::new(board.TIMER0);

	rprintln!("Starting life...");

	let image = [
		[1, 0, 0, 0, 1],
		[0, 1, 0, 1, 0],
		[1, 0, 1, 0, 1],
		[0, 1, 0, 1, 0],
		[1, 0, 0, 0, 1],
	];

    loop {
		display.show(
			&mut timer,
			image,
			5000
		);
		// timer.delay_ms(100);
        rprintln!("{}", counter);
        counter += 1;
    }
}
