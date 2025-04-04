// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use term_grid::{Direction, Filling, Grid, GridOptions};

// This produces:
//
//  1 |  128 |   16384 |   2097152 |   268435456 |   34359738368 |   4398046511104
//  2 |  256 |   32768 |   4194304 |   536870912 |   68719476736 |   8796093022208
//  4 |  512 |   65536 |   8388608 |  1073741824 |  137438953472 |  17592186044416
//  8 | 1024 |  131072 |  16777216 |  2147483648 |  274877906944 |  35184372088832
// 16 | 2048 |  262144 |  33554432 |  4294967296 |  549755813888 |  70368744177664
// 32 | 4096 |  524288 |  67108864 |  8589934592 | 1099511627776 | 140737488355328
// 64 | 8192 | 1048576 | 134217728 | 17179869184 | 2199023255552

fn main() {
    let cells: Vec<_> = (0..48).map(|i| 2_isize.pow(i).to_string()).collect();

    let grid = Grid::new(
        cells,
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Text(" | ".into()),
            width: 80,
        },
    );

    println!("{}", grid);
}
