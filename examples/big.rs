// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

extern crate term_grid;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

fn main() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Text(" | ".into()),
        ..Default::default()
    });

    let mut n: u64 = 1234;
    for _ in 0..50 {
        for _ in 0..10000 {
            grid.add(Cell::from(n.to_string()));
            n = n.overflowing_pow(2).0 % 100000000;
        }

        if let Some(grid_display) = grid.fit_into_width(80) {
            println!("{}", grid_display);
        } else {
            println!("Couldn't fit grid into 80 columns!");
        }
    }
}
