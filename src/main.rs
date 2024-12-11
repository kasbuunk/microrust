#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // take the board
    let board = Board::take().unwrap();
    // make a timer
    let mut timer = hal::Timer::new(board.TIMER0);
    // create the Display
    let mut display = Display::new(board.display_pins);
    // and light up some LEDs
    let grid = block();
    loop {
        display.show(&mut timer, grid, 1000);
        display.clear();
        timer.delay_ms(250);
    }
}

type Grid = [[u8; 5]; 5];

fn block() -> Grid {
    [
        [0, 0, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ]
}

fn heart() -> Grid {
    [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ]
}
