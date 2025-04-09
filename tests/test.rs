// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore underflowed

use term_grid::{Direction, Filling, Grid, GridOptions, DEFAULT_SEPARATOR_SIZE, SPACES_IN_TAB};

#[test]
fn no_items() {
    let grid = Grid::new(
        Vec::<String>::new(),
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 40,
        },
    );

    assert_eq!("", grid.to_string());
}

#[test]
fn one_item() {
    let grid = Grid::new(
        vec!["1"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 40,
        },
    );
    assert_eq!("1\n", grid.to_string());
}

#[test]
fn one_item_exact_width() {
    let grid = Grid::new(
        vec!["1234567890"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 10,
        },
    );

    assert_eq!("1234567890\n", grid.to_string());
}

#[test]
fn one_item_just_over() {
    let grid = Grid::new(
        vec!["1234567890!"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 10,
        },
    );

    assert_eq!(grid.row_count(), 1);
}

#[test]
fn two_small_items() {
    let grid = Grid::new(
        vec!["1", "2"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 40,
        },
    );

    assert_eq!(grid.width(), 1 + 2 + 1);
    assert_eq!("1  2\n", grid.to_string());
}

#[test]
fn two_medium_size_items() {
    let grid = Grid::new(
        vec!["hello there", "how are you today?"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 40,
        },
    );

    assert_eq!(grid.width(), 11 + 2 + 18);
    assert_eq!("hello there  how are you today?\n", grid.to_string());
}

#[test]
fn two_big_items() {
    let grid = Grid::new(
        vec![
            "nuihuneihsoenhisenouiuteinhdauisdonhuisudoiosadiuohnteihaosdinhteuieudi",
            "oudisnuthasuouneohbueobaugceoduhbsauglcobeuhnaeouosbubaoecgueoubeohubeo",
        ],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 40,
        },
    );

    assert_eq!(grid.row_count(), 2);
}

#[test]
fn that_example_from_earlier() {
    let grid = Grid::new(
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve",
        ],
        GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
            width: 24,
        },
    );

    let bits = "one  two three  four\nfive six seven  eight\nnine ten eleven twelve\n";
    assert_eq!(grid.to_string(), bits);
    assert_eq!(grid.row_count(), 3);
}

#[test]
fn number_grid_with_pipe() {
    let grid = Grid::new(
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve",
        ],
        GridOptions {
            filling: Filling::Text("|".into()),
            direction: Direction::LeftToRight,
            width: 24,
        },
    );

    let bits = "one |two|three |four\nfive|six|seven |eight\nnine|ten|eleven|twelve\n";
    assert_eq!(grid.to_string(), bits);
    assert_eq!(grid.row_count(), 3);
}

#[test]
fn huge_separator() {
    let grid = Grid::new(
        vec!["a", "b"],
        GridOptions {
            filling: Filling::Spaces(100),
            direction: Direction::LeftToRight,
            width: 99,
        },
    );
    assert_eq!(grid.row_count(), 2);
}

#[test]
fn huge_yet_unused_separator() {
    let grid = Grid::new(
        vec!["abcd"],
        GridOptions {
            filling: Filling::Spaces(100),
            direction: Direction::LeftToRight,
            width: 99,
        },
    );

    assert_eq!(grid.width(), 4);
    assert_eq!("abcd\n", grid.to_string());
}

// Note: This behaviour is right or wrong depending on your terminal
// This test is mostly added so that we don't change our current
// behaviour, unless we explicitly want to do that.
#[test]
fn emoji() {
    let grid = Grid::new(
        vec!["🦀", "hello", "👩‍🔬", "hello"],
        GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Spaces(2),
            width: 12,
        },
    );
    assert_eq!("🦀    hello\n👩‍🔬  hello\n", grid.to_string());
}

// This test once underflowed, which should never happen. The test is just
// checking that we do not get a panic.
#[test]
fn possible_underflow() {
    let cells: Vec<_> = (0..48).map(|i| 2_isize.pow(i).to_string()).collect();

    let grid = Grid::new(
        cells,
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Text(" | ".into()),
            width: 15,
        },
    );

    println!("{}", grid);
}

#[test]
fn exact_fit() {
    let grid = Grid::new(
        vec!["a", "b", "c", "d"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 4,
        },
    );

    assert_eq!(grid.row_count(), 2);
}

// This is a reproduction of https://github.com/eza-community/eza/issues/845
#[test]
fn eza_many_folders() {
    let cells: Vec<_> = (100000i32..=100401).map(|i| i.to_string()).collect();

    let grid = Grid::new(
        cells,
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 166,
        },
    );

    assert_eq!(grid.row_count(), 20);
}

#[test]
fn filling_with_tabs() {
    let grid = Grid::new(
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve",
        ],
        GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Tabs {
                spaces: DEFAULT_SEPARATOR_SIZE,
                tab_size: 2,
            },
            width: 24,
        },
    );

    let bits = "one\t\t two\t\t three\nfour\t five\t\t six\nseven\t eight\t nine\nten\t\t eleven\t twelve\n";
    assert_eq!(grid.to_string(), bits);
    assert_eq!(grid.row_count(), 4);
}

#[test]
fn padding_bigger_than_widest() {
    let grid = Grid::new(
        vec!["1", "2", "3"],
        GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Tabs {
                spaces: DEFAULT_SEPARATOR_SIZE,
                tab_size: SPACES_IN_TAB,
            },
            width: 20,
        },
    );

    let bits = "1  2  3\n";
    assert_eq!(grid.to_string(), bits);
}

#[test]
fn odd_number_of_entries() {
    let cells = vec!["one", "two", "three", "four", "five"];
    let grid = Grid::new(
        cells.clone(),
        GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Spaces(2),
            width: 15,
        },
    );

    assert_eq!(grid.to_string(), "one    two\nthree  four\nfive\n");

    let grid = Grid::new(
        cells.clone(),
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 15,
        },
    );

    assert_eq!(grid.to_string(), "one    four\ntwo    five\nthree\n");
}

#[test]
fn different_size_separator_with_tabs() {
    let grid = Grid::new(
        vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve",
        ],
        GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Tabs {
                spaces: 4,
                tab_size: 2,
            },
            width: 40,
        },
    );

    let bits = "one\t\t\ttwo\t\t three\t\t four\nfive\t\tsix\t\t seven\t\t eight\nnine\t\tten\t\t eleven\t\t twelve\n";
    assert_eq!(grid.to_string(), bits);
}

#[test]
fn use_max_possible_width() {
    let grid = Grid::new(
        vec![
            "test1", "test2", "test3", "test4", "test5", "test6", "test7", "test8", "test9",
            "test10", "test11",
        ],
        GridOptions {
            filling: Filling::Text("||".to_string()),
            direction: Direction::LeftToRight,
            width: 69,
        },
    );

    let bits = "test1 ||test2 ||test3||test4||test5||test6||test7||test8||test9\ntest10||test11\n";

    assert_eq!(grid.to_string(), bits);
    assert_eq!(grid.row_count(), 2);
}

#[test]
fn dont_use_max_possible_width() {
    let grid = Grid::new(
        vec![
            "test1", "test2", "test3", "test4", "test5", "test6", "test7", "test8", "test9",
            "test10", "test11",
        ],
        GridOptions {
            filling: Filling::Text("||".to_string()),
            direction: Direction::TopToBottom,
            width: 69,
        },
    );

    let bits = "test1||test3||test5||test7||test9 ||test11\ntest2||test4||test6||test8||test10\n";

    assert_eq!(grid.to_string(), bits);
    assert_eq!(grid.row_count(), 2);
}

#[test]
fn use_minimal_optimal_lines() {
    let grid = Grid::new(
        vec!["a", "b", "ccc", "ddd"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 6,
        },
    );

    let expected = "a  ccc\nb  ddd\n";
    assert_eq!(grid.to_string(), expected);
}

#[test]
fn weird_column_edge_case() {
    // Here, 5 columns fit while fewer columns don't. So if we exit too early
    // while increasing columns, we don't find the optimal solution.
    let grid = Grid::new(
        vec!["0", "1", "222222222", "333333333", "4", "5", "6", "7", "8"],
        GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
            width: 21,
        },
    );

    let expected = "\
        0  222222222  4  6  8\n\
        1  333333333  5  7\n\
    ";

    println!("{grid}");
    assert_eq!(grid.to_string(), expected);
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
            let grid = Grid::new(
                vec![
                    "test-width-1",
                    "test-width-2",
                    "test-width-3",
                    "test-width-4",
                ],
                GridOptions {
                    direction: Direction::TopToBottom,
                    filling: Filling::Spaces(2),
                    width,
                },
            );
            assert_eq!(expected, grid.to_string());
        }
    }

    #[test]
    fn across_width_30() {
        let grid = Grid::new(
            vec![
                "test-across1",
                "test-across2",
                "test-across3",
                "test-across4",
            ],
            GridOptions {
                direction: Direction::LeftToRight,
                filling: Filling::Spaces(2),
                width: 30,
            },
        );

        assert_eq!(
            "test-across1  test-across2\ntest-across3  test-across4\n",
            grid.to_string()
        );
    }

    #[test]
    fn columns_width_30() {
        let grid = Grid::new(
            vec![
                "test-columns1",
                "test-columns2",
                "test-columns3",
                "test-columns4",
            ],
            GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Spaces(2),
                width: 30,
            },
        );

        assert_eq!(
            "test-columns1  test-columns3\ntest-columns2  test-columns4\n",
            grid.to_string()
        );
    }

    #[test]
    fn three_short_one_long() {
        let grid = Grid::new(
            vec!["a", "b", "a-long-name", "z"],
            GridOptions {
                direction: Direction::TopToBottom,
                filling: Filling::Spaces(2),
                width: 15,
            },
        );

        assert_eq!("a  a-long-name\nb  z\n", grid.to_string());
    }
}
