// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use term_grid::{Direction, Filling, Grid, GridOptions};

fn main() {
    let mut n: u64 = 1234;
    for _ in 0..50 {
        let mut cells = Vec::new();
        for _ in 0..10000 {
            cells.push(n.to_string());
            n = n.overflowing_pow(2).0 % 100000000;
        }

        let grid = Grid::new(
            cells,
            GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Text(" | ".into()),
                width: 80,
            },
        );

        println!("{grid}");
    }
}
