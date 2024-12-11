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
    let mut grid = block();
    loop {
        display.show(&mut timer, grid, 1000);
        display.clear();
        timer.delay_ms(250);
        grid = next(grid);
    }
}

type Grid = [[u8; 5]; 5];

fn next(grid: Grid) -> Grid {
    let mut updated = [[0; 5]; 5];

    for (index_row, row) in grid.iter().enumerate() {
        for (index_column, cell) in row.iter().enumerate() {
            let num_neighbours = neighbours(grid, index_row, index_column);
            if num_neighbours == 2 || num_neighbours == 3 {
                updated[index_row][index_column] = 1;
            }
        }
    }

    updated
}

fn neighbours(grid: Grid, row: usize, column: usize) -> u8 {
    return (row + column) as u8;
}

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
