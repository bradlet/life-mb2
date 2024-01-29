#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        gpio::{p0, Input, PullDown, PullUp},
        prelude::InputPin,
        timer::Timer,
        Rng,
    },
};

mod life;
use life::{done as is_done, life as execute_life_step};

type BtnAPd = p0::P0_14<Input<PullDown>>;
// Note: For some reason (I'm assuming it's just something about the board config for btn B being different than A)
// this has to be coerced as a `PullUp` instead of `PullDown` pin.
// https://stackoverflow.com/questions/51292571/button-b-on-microbit-always-pushed
type BtnBPd = p0::P0_23<Input<PullUp>>;

const FPS: u32 = 10;
const DELAY_MS: u32 = 1_000 / FPS;
const MAX_STALLED_FRAMES: u32 = 5; // Count of frames where life::done() -> true before randomizing.
const COMPLEMENT_IGNORE_FRAMES: u32 = 5; // Count of frames to ignore further complement operations after pressing btn_b.

/// Take a board and randomly set all pin values.
fn randomize_board<'a>(board: &'a mut [[u8; 5]; 5], rng: &'a mut Rng) {
    for row in 0..board.len() {
        for col in 0..board.len() {
            board[row][col] = if rng.random_u8() >= 128 { 1 } else { 0 };
        }
    }
}

/// Perform a complement operation on the current board state.
fn complement_board(board: &mut [[u8; 5]; 5]) {
    for row in 0..board.len() {
        for col in 0..board.len() {
            let current = board[row][col];
            board[row][col] = if current == 0 { 1 } else { 0 };
        }
    }
}

/// Executes provided `board_fn` if button pressed `Result<bool, Error>` is `true`.
/// Return: `true` if provided closure was executed, else `false`.
fn execute_if_pressed<'a>(
    pressed_result: Option<bool>,
    board: &'a mut [[u8; 5]; 5],
    rng: &'a mut Rng,
    board_fn: fn(&mut [[u8; 5]; 5], &mut Rng) -> (),
) -> bool {
    match pressed_result {
        Some(true) => {
            // Button is pressed
            board_fn(board, rng);
            true
        }
        _ => false,
    }
}

/// Check state of MicroBit buttons and react to button press events.
fn handle_button_events<'a>(
    board: &'a mut [[u8; 5]; 5],
    rng: &'a mut Rng,
    btn_a: &BtnAPd,
    btn_b: &BtnBPd,
    ignore_complement_counter: &mut u32,
) {
    // A:
    execute_if_pressed(btn_a.is_low().ok(), board, rng, randomize_board);
    // B:
    // If we pressed btn_b, then `ignore_complement_counter` will be > 0.
    // No reason to check state of btn_b b/c we are ignoring it.
    if *ignore_complement_counter > 0 {
        rprintln!("Ignoring B: {}", ignore_complement_counter);
        *ignore_complement_counter -= 1;
    } else {
        let executed = execute_if_pressed(btn_b.is_low().ok(), board, rng, |board, _| {
            complement_board(board)
        });
        if executed {
            *ignore_complement_counter = COMPLEMENT_IGNORE_FRAMES;
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut rng = Rng::new(board.RNG);

    let btn_a: BtnAPd = board.buttons.button_a.into_pulldown_input();
    let btn_b: BtnBPd = board.buttons.button_b.into_pullup_input();

    let mut stall_counter = 0u32;
    let mut ignore_complement_counter = 0u32;

    let mut board_img: [[u8; 5]; 5] = [[0; 5]; 5];
    randomize_board(&mut board_img, &mut rng);

    rprintln!("Starting life...");

    loop {
        handle_button_events(
            &mut board_img,
            &mut rng,
            &btn_a,
            &btn_b,
            &mut ignore_complement_counter,
        );

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
