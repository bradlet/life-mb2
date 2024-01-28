#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::InputPin, timer::Timer, Rng},
};

mod life;
use life::{done as is_done, life as execute_life_step};

const FPS: u32 = 10;
const DELAY_MS: u32 = 1_000 / FPS;
const MAX_STALLED_FRAMES: u32 = 20; // Count of frames where life::done() -> true before randomizing.

/// Take a board and randomly set all pin values.
fn randomize_board<'a>(board: &'a mut [[u8; 5]; 5], rng: &'a mut Rng) -> () {
    for row in 0..board.len() {
        for col in 0..board.len() {
            board[row][col] = if rng.random_u8() >= 128 { 1 } else { 0 };
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut stall_counter = 0u32;
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);

    let btn_a = board.buttons.button_a.into_pulldown_input();
    // let btn_b = board.buttons.button_b.into_pulldown_input();

    let mut board_img: [[u8; 5]; 5] = [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ];

    rprintln!("Starting life...");

    loop {
		// Check for button events
        match btn_a.is_low() {
            Ok(true) => {
                // Button is pressed
				randomize_board(&mut board_img, &mut rng);
            }
            Ok(false) => { /* Do nothing */ }
            Err(e) => {
				rprintln!("Error on btn_a: {:?}", e);
			}
        }

        display.show(&mut timer, board_img, DELAY_MS);

		// Execute one step in the Game of Life
		execute_life_step(&mut board_img);

		// If we are (still) stalled, increase stall frame count and check if we need to randomize.
		if is_done(&board_img) {
			stall_counter += 1;
			rprintln!("Stalled frame count: {}", stall_counter);
			if stall_counter >= MAX_STALLED_FRAMES {
				randomize_board(&mut board_img, &mut rng);
				stall_counter = 0;
			}
		} else {
			// Reset stall_counter as we weren't stalled as of this last frame.
			stall_counter = 0;
		}
    }
}
