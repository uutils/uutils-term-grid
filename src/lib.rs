// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#![warn(future_incompatible)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(nonstandard_style)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

use ansi_width::ansi_width;
use std::fmt;

/// Number of spaces in one \t.
pub const SPACES_IN_TAB: usize = 8;

/// Default size for separator in spaces.
pub const DEFAULT_SEPARATOR_SIZE: usize = 2;

/// Direction cells should be written in: either across or downwards.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    /// Starts at the top left and moves rightwards, going back to the first
    /// column for a new row, like a typewriter.
    LeftToRight,

    /// Starts at the top left and moves downwards, going back to the first
    /// row for a new column, like how `ls` lists files by default.
    TopToBottom,
}

/// The text to put in between each pair of columns.
///
/// This does not include any spaces used when aligning cells.
#[derive(PartialEq, Eq, Debug)]
pub enum Filling {
    /// A number of spaces
    Spaces(usize),

    /// An arbitrary string
    ///
    /// `"|"` is a common choice.
    Text(String),

    /// Fill spaces with `\t`
    Tabs {
        /// A number of spaces
        spaces: usize,
        /// Size of `\t` in spaces
        tab_size: usize,
    },
}

impl Filling {
    fn width(&self) -> usize {
        match self {
            Filling::Spaces(w) => *w,
            Filling::Text(t) => ansi_width(t),
            Filling::Tabs { spaces, .. } => *spaces,
        }
    }
}

/// The options for a grid view that should be passed to [`Grid::new`]
#[derive(Debug)]
pub struct GridOptions {
    /// The direction that the cells should be written in
    pub direction: Direction,

    /// The string to put in between each column of cells
    pub filling: Filling,

    /// The width to fill with the grid
    pub width: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Dimensions {
    /// The number of lines in the grid.
    num_lines: usize,

    /// The width of each column in the grid. The length of this vector serves
    /// as the number of columns.
    widths: Vec<usize>,
}

impl Dimensions {
    fn total_width(&self, separator_width: usize) -> usize {
        if self.widths.is_empty() {
            0
        } else {
            let values = self.widths.iter().sum::<usize>();
            let separators = separator_width * (self.widths.len() - 1);
            values + separators
        }
    }
}

/// Everything needed to format the cells with the grid options.
#[derive(Debug)]
pub struct Grid<T: AsRef<str>> {
    options: GridOptions,
    cells: Vec<T>,
    widths: Vec<usize>,
    widest_cell_width: usize,
    dimensions: Dimensions,
}

impl<T: AsRef<str>> Grid<T> {
    /// Creates a new grid view with the given cells and options
    pub fn new(cells: Vec<T>, options: GridOptions) -> Self {
        let widths: Vec<usize> = cells.iter().map(|c| ansi_width(c.as_ref())).collect();
        let widest_cell_width = widths.iter().copied().max().unwrap_or(0);
        let width = options.width;

        let mut grid = Self {
            options,
            cells,
            widths,
            widest_cell_width,
            dimensions: Dimensions {
                num_lines: 0,
                widths: Vec::new(),
            },
        };

        grid.dimensions = grid.width_dimensions(width).unwrap_or(Dimensions {
            num_lines: grid.cells.len(),
            widths: vec![widest_cell_width],
        });

        grid
    }

    /// The number of terminal columns this display takes up, based on the separator
    /// width and the number and width of the columns.
    pub fn width(&self) -> usize {
        self.dimensions.total_width(self.options.filling.width())
    }

    /// The number of rows this display takes up.
    pub fn row_count(&self) -> usize {
        self.dimensions.num_lines
    }

    /// The width of each column
    pub fn column_widths(&self) -> &[usize] {
        &self.dimensions.widths
    }

    /// Returns whether this display takes up as many columns as were allotted
    /// to it.
    ///
    /// It’s possible to construct tables that don’t actually use up all the
    /// columns that they could, such as when there are more columns than
    /// cells! In this case, a column would have a width of zero. This just
    /// checks for that.
    pub fn is_complete(&self) -> bool {
        self.dimensions.widths.iter().all(|&x| x > 0)
    }

    fn compute_dimensions(&self, num_lines: usize, num_columns: usize) -> Dimensions {
        let mut column_widths = vec![0; num_columns];
        for (index, cell_width) in self.widths.iter().copied().enumerate() {
            let index = match self.options.direction {
                Direction::LeftToRight => index % num_columns,
                Direction::TopToBottom => index / num_lines,
            };
            column_widths[index] = cell_width.max(column_widths[index]);
        }

        Dimensions {
            num_lines,
            widths: column_widths,
        }
    }

    fn width_dimensions(&self, maximum_width: usize) -> Option<Dimensions> {
        if self.widest_cell_width > maximum_width {
            // Largest cell is wider than maximum width; it is impossible to fit.
            return None;
        }

        // We compute a maximum bound on the number of rows. This the the number
        // of lines we would have if each cell had the width of the widest element.
        // This allows us to exit the loop below a bit early if we don't find a
        // solution.
        let max_rows = {
            // The first cell doesn't have filling, we just subtract its width
            // and add 1 to the columns.
            let w = maximum_width - self.widest_cell_width;
            let cols = 1 + w / (self.widest_cell_width + self.options.filling.width());

            // Compute the number of lines from the number of columns.
            self.cells.len().div_ceil(cols)
        };

        // Instead of numbers of columns, try to find the fewest number of *rows*
        // that the output will fit in.
        for num_rows in 1..=max_rows {
            // The number of columns is the number of cells divided by the number
            // of lines, rounded up.
            let num_columns = self.cells.len().div_ceil(num_rows);

            // Remove the separator width from the available space.
            //
            // If the the space taken up by the separators is larger than the maximum width
            // we should find a solution with fewer columns, so we continue to the next
            // iteration.
            let total_separator_width = (num_columns - 1) * self.options.filling.width();
            let Some(adjusted_width) = maximum_width.checked_sub(total_separator_width) else {
                continue;
            };

            let potential_dimensions = self.compute_dimensions(num_rows, num_columns);
            if potential_dimensions.widths.iter().sum::<usize>() <= adjusted_width {
                return Some(potential_dimensions);
            }
        }

        None
    }
}

impl<T: AsRef<str>> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // If cells are empty then, nothing to print, skip.
        if self.cells.is_empty() {
            return Ok(());
        }

        let (tab_size, separator) = match &self.options.filling {
            Filling::Spaces(n) => (0, " ".repeat(*n)),
            Filling::Text(s) => (0, s.clone()),
            Filling::Tabs { spaces, tab_size } => (*tab_size, " ".repeat(*spaces)),
        };

        // Initialize a buffer of spaces. The idea here is that any cell
        // that needs padding gets a slice of this buffer of the needed
        // size. This avoids the need of creating a string of spaces for
        // each cell that needs padding.
        //
        // We overestimate how many spaces we need, but this is not
        // part of the loop and it's therefore not super important to
        // get exactly right.
        let padding = " ".repeat(self.widest_cell_width + self.options.filling.width());

        for y in 0..self.dimensions.num_lines {
            // Current position on the line.
            let mut cursor: usize = 0;
            for x in 0..self.dimensions.widths.len() {
                // Calculate position of the current element of the grid
                // in cells and widths vectors and the offset to the next value.
                let (current, offset) = match self.options.direction {
                    Direction::LeftToRight => (y * self.dimensions.widths.len() + x, 1),
                    Direction::TopToBottom => {
                        (y + self.dimensions.num_lines * x, self.dimensions.num_lines)
                    }
                };

                // Abandon a line mid-way through if that’s where the cells end.
                if current >= self.cells.len() {
                    break;
                }

                // Last in row checks only the predefined grid width.
                // It does not check if there will be more entries.
                // For this purpose we define next value as well.
                // This prevents printing separator after the actual last value in a row.
                let last_in_row = x == self.dimensions.widths.len() - 1;
                let contents = &self.cells[current];
                let width = self.widths[current];
                let col_width = self.dimensions.widths[x];
                let padding_size = col_width - width;

                // The final column doesn’t need to have trailing spaces,
                // as long as it’s left-aligned.
                //
                // We use write_str directly instead of a the write! macro to
                // avoid some of the formatting overhead. For example, if we pad
                // using `write!("{contents:>width}")`, the unicode width will
                // have to be independently calculated by the macro, which is slow and
                // redundant because we already know the width.
                //
                // For the padding, we instead slice into a buffer of spaces defined
                // above, so we don't need to call `" ".repeat(n)` each loop.
                // We also only call `write_str` when we actually need padding as
                // another optimization.
                f.write_str(contents.as_ref())?;

                // In case this entry was the last on the current line,
                // there is no need to print the separator and padding.
                if last_in_row || current + offset >= self.cells.len() {
                    break;
                }

                // Special case if tab size was not set. Fill with spaces and separator.
                if tab_size == 0 {
                    f.write_str(&padding[..padding_size])?;
                    f.write_str(&separator)?;
                } else {
                    // Move cursor to the end of the current contents.
                    cursor += width;
                    let total_spaces = padding_size + self.options.filling.width();
                    // The size of \t can be inconsistent in terminal.
                    // Tab stops are relative to the cursor position e.g.,
                    //  * cursor = 0, \t moves to column 8;
                    //  * cursor = 5, \t moves to column 8 (3 spaces);
                    //  * cursor = 9, \t moves to column 16 (7 spaces).
                    // Calculate the nearest \t position in relation to cursor.
                    let closest_tab = tab_size - (cursor % tab_size);

                    if closest_tab > total_spaces {
                        f.write_str(&padding[..total_spaces])?;
                    } else {
                        let rest_spaces = total_spaces - closest_tab;
                        let tabs = 1 + (rest_spaces / tab_size);
                        let spaces = rest_spaces % tab_size;
                        f.write_str(&"\t".repeat(tabs))?;
                        f.write_str(&padding[..spaces])?;
                    }

                    cursor += total_spaces;
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}
