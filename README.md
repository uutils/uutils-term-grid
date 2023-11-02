[![Crates.io](https://img.shields.io/crates/v/uutils-term-grid.svg)](https://crates.io/crates/uutils-term-grid)
[![dependency status](https://deps.rs/repo/github/uutils/uutils-term-grid/status.svg)](https://deps.rs/repo/github/uutils/uutils-term-grid)
[![CodeCov](https://codecov.io/gh/uutils/uutils-term-grid/branch/master/graph/badge.svg)](https://codecov.io/gh/uutils/uutils-term-grid)

# uutils-term-grid

This library arranges textual data in a grid format suitable for fixed-width fonts, using an algorithm to minimise the amount of space needed.

---

This library is forked from the [`rust-term-grid`](https://github.com/ogham/rust-term-grid) library.

---

# Installation

This crate works with `cargo`. Add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
uutils_term_grid = "0.3"
```

The Minimum Supported Rust Version is 1.70.

This library arranges textual data in a grid format suitable for
fixed-width fonts, using an algorithm to minimise the amount of space
needed. For example:

```rust
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};

let cells = vec![
    "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine", "ten", "eleven", "twelve"
];

let grid = Grid::new(
    cells,
    GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
        width: 24,
    }
);

println!("{grid}");
```

Produces the following tabular result:

```text
one  two three  four
five six seven  eight
nine ten eleven twelve
```

## Creating a grid

To add data to a grid, first create a new [`Grid`] value with a list of strings and a set of options.

There are three options that must be specified in the [`GridOptions`] value
that dictate how the grid is formatted:

- [`filling`](struct.GridOptions.html#structfield.direction): what to put in between two columns — either a number of
  spaces, or a text string;
- [`direction`](struct.GridOptions.html#structfield.direction): specifies whether the cells should go along
  rows, or columns:
  - `Direction::LeftToRight` starts them in the top left and
    moves _rightwards_, going to the start of a new row after reaching the
    final column;
  - `Direction::TopToBottom` starts them in the top left and moves
    _downwards_, going to the top of a new column after reaching the final
    row.
- [`width`](struct.GridOptions.html#structfield.direction): the width to fill the grid into. Usually, this should be the width
  of the terminal.

## Cells and data

Grids to not take [`String`]s or `&str`s — they take [`Cell`] values.

A [`Cell`] is a struct containing an individual cell’s contents, as a string,
and its pre-computed length, which gets used when calculating a grid’s final
dimensions. Usually, you want the _Unicode width_ of the string to be used for
this, so you can turn a [`String`] into a [`Cell`] with the `.into()` function.

However, you may also want to supply your own width: when you already know the
width in advance, or when you want to change the measurement, such as skipping
over terminal control characters. For cases like these, the fields on the
[`Cell`] values are public, meaning you can construct your own instances as
necessary.
