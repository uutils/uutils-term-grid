use term_grid::{Alignment, Cell, Direction, Filling, Grid, GridOptions};

#[test]
fn no_items() {
    let grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    let display = grid.fit_into_width(40).unwrap();
    assert_eq!("", display.to_string());
}

#[test]
fn one_item() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from("1"));

    let display = grid.fit_into_width(40).unwrap();
    assert_eq!("1\n", display.to_string());
}

#[test]
fn one_item_exact_width() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from("1234567890"));

    let display = grid.fit_into_width(10).unwrap();
    assert_eq!("1234567890\n", display.to_string());
}

#[test]
fn one_item_just_over() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from("1234567890!"));

    assert!(grid.fit_into_width(10).is_none());
}

#[test]
fn two_small_items() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from("1"));
    grid.add(Cell::from("2"));

    let display = grid.fit_into_width(40).unwrap();

    assert_eq!(display.width(), 1 + 2 + 1);
    assert_eq!("1  2\n", display.to_string());
}

#[test]
fn two_medium_size_items() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from("hello there"));
    grid.add(Cell::from("how are you today?"));

    let display = grid.fit_into_width(40).unwrap();

    assert_eq!(display.width(), 11 + 2 + 18);
    assert_eq!("hello there  how are you today?\n", display.to_string());
}

#[test]
fn two_big_items() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::TopToBottom,
        filling: Filling::Spaces(2),
    });

    grid.add(Cell::from(
        "nuihuneihsoenhisenouiuteinhdauisdonhuisudoiosadiuohnteihaosdinhteuieudi",
    ));
    grid.add(Cell::from(
        "oudisnuthasuouneohbueobaugceoduhbsauglcobeuhnaeouosbubaoecgueoubeohubeo",
    ));

    assert!(grid.fit_into_width(40).is_none());
}

#[test]
fn that_example_from_earlier() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for s in &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ] {
        grid.add(Cell::from(*s));
    }

    let bits = "one  two three  four\nfive six seven  eight\nnine ten eleven twelve\n";
    assert_eq!(grid.fit_into_width(24).unwrap().to_string(), bits);
    assert_eq!(grid.fit_into_width(24).unwrap().row_count(), 3);
}

#[test]
fn number_grid_with_pipe() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Text("|".into()),
        direction: Direction::LeftToRight,
    });

    for s in &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ] {
        grid.add(Cell::from(*s));
    }

    let bits = "one |two|three |four\nfive|six|seven |eight\nnine|ten|eleven|twelve\n";
    assert_eq!(grid.fit_into_width(24).unwrap().to_string(), bits);
    assert_eq!(grid.fit_into_width(24).unwrap().row_count(), 3);
}

#[test]
fn numbers_right() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for s in &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ] {
        let mut cell = Cell::from(*s);
        cell.alignment = Alignment::Right;
        grid.add(cell);
    }

    let bits = " one two  three   four\nfive six  seven  eight\nnine ten eleven twelve\n";
    assert_eq!(grid.fit_into_width(24).unwrap().to_string(), bits);
    assert_eq!(grid.fit_into_width(24).unwrap().row_count(), 3);
}

#[test]
fn numbers_right_pipe() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Text("|".into()),
        direction: Direction::LeftToRight,
    });

    for s in &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ] {
        let mut cell = Cell::from(*s);
        cell.alignment = Alignment::Right;
        grid.add(cell);
    }

    let bits = " one|two| three|  four\nfive|six| seven| eight\nnine|ten|eleven|twelve\n";
    assert_eq!(grid.fit_into_width(24).unwrap().to_string(), bits);
    assert_eq!(grid.fit_into_width(24).unwrap().row_count(), 3);
}

#[test]
fn huge_separator() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(100),
        direction: Direction::LeftToRight,
    });

    grid.add("a".into());
    grid.add("b".into());

    assert!(grid.fit_into_width(99).is_none());
}

#[test]
fn huge_yet_unused_separator() {
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(100),
        direction: Direction::LeftToRight,
    });

    grid.add("abcd".into());

    let display = grid.fit_into_width(99).unwrap();

    assert_eq!(display.width(), 4);
    assert_eq!("abcd\n", display.to_string());
}

// Note: This behaviour is right or wrong depending on your terminal
// This test is mostly added so that we don't change our current
// behaviour, unless we explictly want to do that.
#[test]
fn emoji() {
    let mut grid = Grid::new(GridOptions {
        direction: Direction::LeftToRight,
        filling: Filling::Spaces(2),
    });

    for s in ["hello", "🦀", "👩‍🔬"] {
        let mut cell = Cell::from(s);
        cell.alignment = Alignment::Right;
        grid.add(cell);
    }

    let display = grid.fit_into_width(7).unwrap();
    assert_eq!("hello\n   🦀\n 👩‍🔬\n", display.to_string());
}

// These test are based on the tests in uutils ls, to ensure we won't break
// it while editing this library.
mod uutils_ls {
    use super::*;

    #[test]
    fn different_widths() {
        for (width, expected) in [
            (
                100,
                "test-width-1  test-width-2  test-width-3  test-width-4\n",
            ),
            (
                50,
                "test-width-1  test-width-3\ntest-width-2  test-width-4\n",
            ),
            (
                25,
                "test-width-1\ntest-width-2\ntest-width-3\ntest-width-4\n",
            ),
        ] {
            let mut grid = Grid::new(GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Spaces(2),
            });

            for s in [
                "test-width-1",
                "test-width-2",
                "test-width-3",
                "test-width-4",
            ] {
                let mut cell = Cell::from(s);
                cell.alignment = Alignment::Left;
                grid.add(cell);
            }

            let display = grid.fit_into_width(width).unwrap();
            assert_eq!(expected, display.to_string());
        }
    }

    #[test]
    fn across_width_30() {
        let mut grid = Grid::new(GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Spaces(2),
        });

        for s in [
            "test-across1",
            "test-across2",
            "test-across3",
            "test-across4",
        ] {
            let mut cell = Cell::from(s);
            cell.alignment = Alignment::Left;
            grid.add(cell);
        }

        let display = grid.fit_into_width(30).unwrap();
        assert_eq!(
            "test-across1  test-across2\ntest-across3  test-across4\n",
            display.to_string()
        );
    }

    #[test]
    fn columns_width_30() {
        let mut grid = Grid::new(GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
        });

        for s in [
            "test-columns1",
            "test-columns2",
            "test-columns3",
            "test-columns4",
        ] {
            let mut cell = Cell::from(s);
            cell.alignment = Alignment::Left;
            grid.add(cell);
        }

        let display = grid.fit_into_width(30).unwrap();
        assert_eq!(
            "test-columns1  test-columns3\ntest-columns2  test-columns4\n",
            display.to_string()
        );
    }

    #[test]
    fn three_short_one_long() {
        let mut grid = Grid::new(GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
        });

        for s in ["a", "b", "a-long-name", "z"] {
            let mut cell = Cell::from(s);
            cell.alignment = Alignment::Left;
            grid.add(cell);
        }

        let display = grid.fit_into_width(15).unwrap();
        assert_eq!("a  a-long-name\nb  z\n", display.to_string());
    }
}
