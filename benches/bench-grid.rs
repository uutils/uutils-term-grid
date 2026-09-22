// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::hint::black_box;

use divan::Bencher;
use term_grid::{Direction, Filling, Grid, GridOptions};

const CELL_COUNTS: [usize; 4] = [10, 100, 1_000, 10_000];

fn generate_cells(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("file-{i:04}.txt")).collect()
}

#[divan::bench(args = CELL_COUNTS)]
fn grid_top_to_bottom(bencher: Bencher, n: usize) {
    let cells = generate_cells(n);
    bencher.with_inputs(|| cells.clone()).bench_values(|cells| {
        black_box(Grid::new(
            cells,
            GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Spaces(2),
                width: 80,
            },
        ))
    });
}

#[divan::bench(args = CELL_COUNTS)]
fn grid_left_to_right(bencher: Bencher, n: usize) {
    let cells = generate_cells(n);
    bencher.with_inputs(|| cells.clone()).bench_values(|cells| {
        black_box(Grid::new(
            cells,
            GridOptions {
                direction: Direction::LeftToRight,
                filling: Filling::Spaces(2),
                width: 80,
            },
        ))
    });
}

#[divan::bench(args = CELL_COUNTS)]
fn grid_display(bencher: Bencher, n: usize) {
    let cells = generate_cells(n);
    let grid = Grid::new(
        cells,
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 80,
        },
    );
    bencher.bench(|| black_box(grid.to_string()));
}

#[divan::bench(args = [20, 80, 160, 320])]
fn grid_varying_width(bencher: Bencher, width: usize) {
    let cells = generate_cells(500);
    bencher.with_inputs(|| cells.clone()).bench_values(|cells| {
        black_box(Grid::new(
            cells,
            GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Spaces(2),
                width,
            },
        ))
    });
}

fn main() {
    divan::main();
}
