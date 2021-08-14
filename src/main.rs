use std::{
    convert::TryFrom,
    io::{self, BufRead},
    num::NonZeroU8,
    str::FromStr,
};

type Grid = [[Option<NonZeroU8>; 9]; 9];

fn read_input<T: BufRead>(mut input: T) -> io::Result<Grid> {
    let mut grid: [[Option<NonZeroU8>; 9]; 9] = [[None; 9]; 9];

    let mut line_number = 0;
    loop {
        let mut line = String::new();
        input.read_line(&mut line)?;

        let input = line.trim();

        if input.len() == 0 {
            break;
        }

        let values = input.split_ascii_whitespace().enumerate();

        for (index, value) in values {
            let value = NonZeroU8::from_str(value).ok();
            if let Some(n) = value {
                if u8::from(n) > 9u8 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Cell value is greater than 9"));
                }
            }
            grid[line_number][index] = value;
        }

        line_number += 1;
    }

    Ok(grid)
}

fn get_unseen_values(seen_values: [bool; 9]) -> Vec<NonZeroU8> {
    seen_values
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if !v {
                let uint = u8::try_from(i).expect("seen value index should be < 9");
                NonZeroU8::new(uint + 1)
            } else {
                None
            }
        })
        .collect()
}

fn get_box_indices(index: usize) -> (usize, usize) {
    if index < 3 {
        (0, 3)
    } else if index < 6 {
        (3, 6)
    } else {
        (6, 9)
    }
}

fn get_box_values(grid: &Grid, row_index: usize, column_index: usize) -> [Option<NonZeroU8>; 9] {
    let (box_start_row, box_stop_row) = get_box_indices(row_index);
    let (box_start_column, box_stop_column) = get_box_indices(column_index);

    let mut values: [Option<NonZeroU8>; 9] = [None; 9];

    let rows = &grid[box_start_row..box_stop_row];

    let mut index = 0;
    for row in rows.iter() {
        for cell in &row[box_start_column..box_stop_column] {
            values[index] = *cell;
            index += 1;
        }
    }

    values
}

fn get_possible_values(grid: &Grid, row_index: usize, column_index: usize) -> Vec<NonZeroU8> {
    // Possible values depend on the other values in the column, row and box.

    let mut seen_values: [bool; 9] = [false; 9];

    // Check the row.
    for value in grid[row_index] {
        if let Some(n) = value {
            seen_values[usize::from(u8::from(n)) - 1] = true;
        }
    }

    // Check the column.
    let values = grid.iter().filter_map(|row| row[column_index]);

    for value in values {
        seen_values[usize::from(u8::from(value)) - 1] = true;
    }

    // Check the box.
    let box_values = get_box_values(grid, row_index, column_index);
    for value in box_values {
        if let Some(n) = value {
            seen_values[usize::from(u8::from(n)) - 1] = true;
        }
    }

    get_unseen_values(seen_values)
}

fn is_solved(grid: &Grid) -> bool {
    grid.iter().flatten().all(Option::is_some)
}

fn print_grid(grid: &Grid) -> String {
    let mut output = String::with_capacity(9 * 9 * 2);

    for row in grid {
        for (index, cell) in row.iter().enumerate() {
            if let Some(n) = cell {
                output.push_str(&n.to_string());
            } else {
                output.push('_');
            }
            if index < 8 {
                output.push(' ');
            } else {
                output.push_str("\n");
            }
        }
    }

    output
}

fn solve(grid: Grid) -> Grid {
    let mut working_grid = grid;
    let mut loop_count = 0;
    loop {
        if is_solved(&working_grid) {
            return working_grid;
        }

        let mut new_grid = working_grid.clone();

        'outer: for (row_index, row) in working_grid.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                if cell.is_some() {
                    continue;
                }

                let possible_values = get_possible_values(&working_grid, row_index, column_index);

                // println!("Possible values for ({}, {}): {:?}", row_index, column_index, possible_values);

                if possible_values.is_empty() {
                    panic!(
                        "The cell at ({}, {}) has no possible values! Current grid:\n{}",
                        row_index,
                        column_index,
                        print_grid(&working_grid)
                    );
                }

                if possible_values.len() == 1 {
                    // println!(
                    //     "Set value for ({}, {}): {:?}",
                    //     row_index, column_index, possible_values
                    // );
                    new_grid[row_index][column_index] = Some(possible_values[0]);
                    break 'outer;
                }
            }
        }

        if working_grid == new_grid {
            panic!("Infinite loop, grid is unchanged after {} loops:\n{}", loop_count, print_grid(&new_grid));
        }

        // println!("Replacing working grid with:\n{}", print_grid(&new_grid));

        working_grid = new_grid;
        loop_count += 1;
    }
}

fn main() -> io::Result<()> {
    let grid = read_input(io::stdin().lock())?;

    println!("Read values:\n{}", print_grid(&grid));

    let solved_grid = solve(grid);

    println!("Solved values:\n{}", print_grid(&solved_grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_should_split_by_whitespace_and_treat_non_numeric_values_as_empties() {
        fn n(v: u8) -> Option<NonZeroU8> {
            NonZeroU8::new(v)
        }

        let input =   "_ 6 2 _ 9 8 1 _ 4\n\
                            5 _ _ 2 _ 1 _ 9 _\n\
                            _ 8 _ _ _ 3 _ 5 7\n\
                            4 _ 1 7 _ _ _ 8 _\n\
                            _ _ _ _ _ _ _ _ _\n\
                            _ 7 _ _ _ 9 6 _ 2\n\
                            3 1 _ 9 _ _ _ 6 _\n\
                            _ 5 _ 6 _ 4 _ _ 9\n\
                            6 _ 8 3 1 _ 4 2 _\n\n";

        let expected_output =  [
            [None, n(6), n(2), None, n(9), n(8), n(1), None, n(4)],
            [n(5), None, None, n(2), None, n(1), None, n(9), None],
            [None, n(8), None, None, None, n(3), None, n(5), n(7)],
            [n(4), None, n(1), n(7), None, None, None, n(8), None],
            [None, None, None, None, None, None, None, None, None],
            [None, n(7), None, None, None, n(9), n(6), None, n(2)],
            [n(3), n(1), None, n(9), None, None, None, n(6), None],
            [None, n(5), None, n(6), None, n(4), None, None, n(9)],
            [n(6), None, n(8), n(3), n(1), None, n(4), n(2), None],
        ];

        let output = read_input(input.as_bytes()).unwrap();

        assert_eq!(expected_output, output);
    }

    #[test]
    fn solve_should_complete_successfully() {
        let grids = [
            (
                "_ 6 2 _ 9 8 1 _ 4\n\
                5 _ _ 2 _ 1 _ 9 _\n\
                _ 8 _ _ _ 3 _ 5 7\n\
                4 _ 1 7 _ _ _ 8 _\n\
                _ _ _ _ _ _ _ _ _\n\
                _ 7 _ _ _ 9 6 _ 2\n\
                3 1 _ 9 _ _ _ 6 _\n\
                _ 5 _ 6 _ 4 _ _ 9\n\
                6 _ 8 3 1 _ 4 2 _\n\n",

                "7 6 2 5 9 8 1 3 4\n\
                5 4 3 2 7 1 8 9 6\n\
                1 8 9 4 6 3 2 5 7\n\
                4 2 1 7 5 6 9 8 3\n\
                9 3 6 8 4 2 5 7 1\n\
                8 7 5 1 3 9 6 4 2\n\
                3 1 4 9 2 5 7 6 8\n\
                2 5 7 6 8 4 3 1 9\n\
                6 9 8 3 1 7 4 2 5\n"
            ),
            (
                "_ 6 7 5 2 _ _ _ _\n\
                1 _ _ 9 4 _ 7 _ _\n\
                _ _ 4 _ _ _ _ 1 _\n\
                9 _ _ _ 8 7 _ 5 3\n\
                _ 8 6 _ 3 _ 9 7 _\n\
                5 7 _ 2 9 _ _ _ 4\n\
                _ 5 _ _ _ _ 4 _ _\n\
                _ _ 1 _ 7 4 _ _ 8\n\
                _ _ _ _ 5 9 2 6 _\n\n",

                "8 6 7 5 2 1 3 4 9\n\
                1 3 5 9 4 8 7 2 6\n\
                2 9 4 7 6 3 8 1 5\n\
                9 1 2 4 8 7 6 5 3\n\
                4 8 6 1 3 5 9 7 2\n\
                5 7 3 2 9 6 1 8 4\n\
                6 5 9 8 1 2 4 3 7\n\
                3 2 1 6 7 4 5 9 8\n\
                7 4 8 3 5 9 2 6 1\n"
            )
        ];

        for (input, expected_output) in grids {
            let parsed = read_input(input.as_bytes()).unwrap();
            let solved = solve(parsed);

            assert_eq!(expected_output, print_grid(&solved));
        }
    }
}
