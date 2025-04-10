[![Crates.io](https://img.shields.io/crates/v/uutils-term-grid.svg)](https://crates.io/crates/uutils-term-grid)
[![dependency status](https://deps.rs/repo/github/uutils/uutils-term-grid/status.svg)](https://deps.rs/repo/github/uutils/uutils-term-grid)
[![CodeCov](https://codecov.io/gh/uutils/uutils-term-grid/branch/master/graph/badge.svg)](https://codecov.io/gh/uutils/uutils-term-grid)

# uutils-term-grid

This library arranges textual data in a grid format suitable for fixed-width
fonts, using an algorithm to minimise the amount of space needed.

---

This library is forked from the unmaintained
[`rust-term-grid`](https://github.com/ogham/rust-term-grid) library. The core
functionality has remained the same, with some additional bugfixes, performance
improvements and a new API.

---

# Installation

This crate works with `cargo`. Add the following to your `Cargo.toml`
dependencies section:

```toml
[dependencies]
uutils_term_grid = "0.7"
```

The Minimum Supported Rust Version is 1.70.

## Creating a grid

To add data to a grid, first create a new [`Grid`] value with a list of strings
and a set of options.

There are three options that must be specified in the [`GridOptions`] value that
dictate how the grid is formatted:

- [`filling`][filling]: how to fill empty space between columns:
  - [`Filling::Spaces`][Spaces] number of spaces between columns;
  - [`Filling::Text`][Text] text string separator between columns;
  - [`Filling::Tabs`][Tabs] special option which allows to set number of spaces between columns and set the size of `\t` character.
- [`direction`][direction]: specifies whether the cells should go along rows, or
  columns:
  - [`Direction::LeftToRight`][LeftToRight] starts them in the top left and
    moves _rightwards_, going to the start of a new row after reaching the final
    column;
  - [`Direction::TopToBottom`][TopToBottom] starts them in the top left and
    moves _downwards_, going to the top of a new column after reaching the final
    row.
- [`width`][width]: the width to fill the grid into. Usually, this should be the
  width of the terminal.

In practice, creating a grid can be done as follows:

```rust
use term_grid::{Grid, GridOptions, Direction, Filling};

// Create a `Vec` of text to put in the grid
let cells = vec![
    "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine", "ten", "eleven", "twelve"
];

// Then create a `Grid` with those cells.
// The grid requires several options:
//  - The filling determines the string used as separator
//    between the columns.
//  - The direction specifies whether the layout should
//    be done row-wise or column-wise.
//  - The width is the maximum width that the grid might
//    have.
let grid = Grid::new(
    cells,
    GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
        width: 24,
    }
);

// A `Grid` implements `Display` and can be printed directly.
println!("{grid}");
```

Produces the following tabular result:

```text
one  two three  four
five six seven  eight
nine ten eleven twelve
```

[filling]: https://docs.rs/uutils_term_grid/latest/term_grid/struct.GridOptions.html#structfield.filling
[direction]: https://docs.rs/uutils_term_grid/latest/term_grid/struct.GridOptions.html#structfield.direction
[width]: https://docs.rs/uutils_term_grid/latest/term_grid/struct.GridOptions.html#structfield.width
[LeftToRight]: https://docs.rs/uutils_term_grid/latest/term_grid/enum.Direction.html#variant.LeftToRight
[TopToBottom]: https://docs.rs/uutils_term_grid/latest/term_grid/enum.Direction.html#variant.TopToBottom
[Spaces]: https://docs.rs/uutils_term_grid/latest/term_grid/enum.Filling.html#variant.Spaces
[Text]: https://docs.rs/uutils_term_grid/latest/term_grid/enum.Filling.html#variant.Text
[Tabs]:https://docs.rs/uutils_term_grid/latest/term_grid/enum.Filling.html#variant.Tabs

## Width of grid cells

This library calculates the width of strings as displayed in the terminal using
the [`ansi-width`][ansi-width] crate. This takes into account the width of
characters and ignores ANSI codes.

The width calculation is currently not configurable. If you have a use-case for
which this calculation is wrong, please open an issue.

[ansi-width]: https://docs.rs/ansi-width
