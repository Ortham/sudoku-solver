use std::{
    io::{self, BufRead},
    num::NonZeroU8,
    str::FromStr,
};

type Grid = [[Option<NonZeroU8>; 9]; 9];

fn read_input<T: BufRead>(mut input: T) -> io::Result<Grid> {
    let mut grid: [[Option<NonZeroU8>; 9]; 9] = [[None; 9]; 9];

    for row in grid.iter_mut() {
        let mut line = String::new();
        input.read_line(&mut line)?;

        let values = line
            .split_ascii_whitespace()
            .map(|value| NonZeroU8::from_str(value).ok())
            .enumerate();

        let mut value_count = 0;
        for (index, value) in values {
            if let Some(n) = value.map(u8::from) {
                if n > 9u8 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Cell value is greater than 9",
                    ));
                }
            }

            row[index] = value;

            value_count += 1;
        }

        if value_count != 9 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Line does not contain 9 values",
            ));
        }
    }

    Ok(grid)
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

fn get_box_values(
    grid: &Grid,
    row_index: usize,
    column_index: usize,
) -> impl Iterator<Item = &NonZeroU8> {
    let (box_start_row, box_stop_row) = get_box_indices(row_index);
    let (box_start_column, box_stop_column) = get_box_indices(column_index);

    grid[box_start_row..box_stop_row]
        .iter()
        .map(move |row| &row[box_start_column..box_stop_column])
        .flatten()
        .flatten()
}

fn get_possible_values(
    grid: &Grid,
    row_index: usize,
    column_index: usize,
) -> [Option<NonZeroU8>; 9] {
    // Possible values depend on the other values in the column, row and box.
    let mut unseen_values: [Option<NonZeroU8>; 9] = [
        Some(NonZeroU8::new(1).expect("1 is non-zero")),
        Some(NonZeroU8::new(2).expect("2 is non-zero")),
        Some(NonZeroU8::new(3).expect("3 is non-zero")),
        Some(NonZeroU8::new(4).expect("4 is non-zero")),
        Some(NonZeroU8::new(5).expect("5 is non-zero")),
        Some(NonZeroU8::new(6).expect("6 is non-zero")),
        Some(NonZeroU8::new(7).expect("7 is non-zero")),
        Some(NonZeroU8::new(8).expect("8 is non-zero")),
        Some(NonZeroU8::new(9).expect("9 is non-zero")),
    ];

    // Check the row.
    for value in grid[row_index].iter().flatten() {
        unseen_values[usize::from(value.get()) - 1] = None;
    }

    // Check the column.
    let values = grid.iter().filter_map(|row| row[column_index]);

    for value in values {
        unseen_values[usize::from(value.get()) - 1] = None;
    }

    // Check the box.
    let box_values = get_box_values(grid, row_index, column_index);
    for value in box_values {
        unseen_values[usize::from(value.get()) - 1] = None;
    }

    unseen_values
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
                output.push('\n');
            }
        }
    }

    output
}

fn solve(grid: Grid) -> Grid {
    let mut grid_stack = vec![grid];

    let mut loop_count = 0;
    loop {
        let working_grid = match grid_stack.pop() {
            Some(g) => g,
            None => panic!("Grid stack is empty, solver is stuck!"),
        };

        if is_solved(&working_grid) {
            println!("Solved sudoku in {} loops!", loop_count);
            return working_grid;
        }

        'outer: for (row_index, row) in working_grid.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                if cell.is_some() {
                    continue;
                }

                let possible_values = get_possible_values(&working_grid, row_index, column_index);

                for possible_value in possible_values {
                    if possible_value.is_some() {
                        // For each possible value, create a new grid and add it to
                        // the stack, so that each grid is independently looped over
                        let mut new_grid = working_grid;
                        new_grid[row_index][column_index] = possible_value;

                        grid_stack.push(new_grid);
                    }
                }

                // Either the solver got stuck and couldn't find any possible
                // values, in which case the current grid should be abandoned,
                // or it added new grid(s) to the stack, in which case they
                // should now be attempted.
                break 'outer;
            }
        }

        loop_count += 1;
    }
}

fn main() -> io::Result<()> {
    let grid = read_input(io::stdin().lock())?;

    let start = std::time::Instant::now();

    let solved_grid = solve(grid);

    let elapsed = start.elapsed();
    println!("Took {} ms to solve", elapsed.as_millis());

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

        let input = "_ 6 2 _ 9 8 1 _ 4\n\
                            5 _ _ 2 _ 1 _ 9 _\n\
                            _ 8 _ _ _ 3 _ 5 7\n\
                            4 _ 1 7 _ _ _ 8 _\n\
                            _ _ _ _ _ _ _ _ _\n\
                            _ 7 _ _ _ 9 6 _ 2\n\
                            3 1 _ 9 _ _ _ 6 _\n\
                            _ 5 _ 6 _ 4 _ _ 9\n\
                            6 _ 8 3 1 _ 4 2 _\n";

        let expected_output = [
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
                6 _ 8 3 1 _ 4 2 _\n",
                "7 6 2 5 9 8 1 3 4\n\
                5 4 3 2 7 1 8 9 6\n\
                1 8 9 4 6 3 2 5 7\n\
                4 2 1 7 5 6 9 8 3\n\
                9 3 6 8 4 2 5 7 1\n\
                8 7 5 1 3 9 6 4 2\n\
                3 1 4 9 2 5 7 6 8\n\
                2 5 7 6 8 4 3 1 9\n\
                6 9 8 3 1 7 4 2 5\n",
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
                _ _ _ _ 5 9 2 6 _\n",
                "8 6 7 5 2 1 3 4 9\n\
                1 3 5 9 4 8 7 2 6\n\
                2 9 4 7 6 3 8 1 5\n\
                9 1 2 4 8 7 6 5 3\n\
                4 8 6 1 3 5 9 7 2\n\
                5 7 3 2 9 6 1 8 4\n\
                6 5 9 8 1 2 4 3 7\n\
                3 2 1 6 7 4 5 9 8\n\
                7 4 8 3 5 9 2 6 1\n",
            ),
            (
                "9 _ _ _ _ 7 _ _ 3\n\
                _ _ 3 _ _ 1 8 _ _\n\
                _ _ _ _ 8 _ _ _ 7\n\
                7 _ _ _ _ _ 5 _ _\n\
                4 _ 8 _ 5 _ 7 _ 2\n\
                _ _ 2 _ _ _ _ _ 1\n\
                5 _ _ _ 9 _ _ _ _\n\
                _ _ 1 7 _ _ 2 _ _\n\
                2 _ _ 6 _ _ _ _ 4\n",
                "9 8 5 2 6 7 4 1 3\n\
                6 7 3 9 4 1 8 2 5\n\
                1 2 4 5 8 3 6 9 7\n\
                7 1 9 3 2 4 5 8 6\n\
                4 6 8 1 5 9 7 3 2\n\
                3 5 2 8 7 6 9 4 1\n\
                5 3 6 4 9 2 1 7 8\n\
                8 4 1 7 3 5 2 6 9\n\
                2 9 7 6 1 8 3 5 4\n",
            ),
            (
                "_ _ _ _ _ _ _ 2 5\n\
                _ 9 4 _ 2 _ _ _ _\n\
                5 _ 7 _ _ 3 _ _ 1\n\
                1 _ _ _ _ 5 7 _ _\n\
                _ 3 _ _ _ _ _ 4 _\n\
                _ _ 8 4 _ _ _ _ 9\n\
                4 _ _ 7 _ _ 8 _ 2\n\
                _ _ _ _ 5 _ 9 1 _\n\
                2 6 _ _ _ _ _ _ _\n",
                "8 1 6 9 4 7 3 2 5\n\
                3 9 4 5 2 1 6 8 7\n\
                5 2 7 6 8 3 4 9 1\n\
                1 4 2 8 9 5 7 6 3\n\
                9 3 5 1 7 6 2 4 8\n\
                6 7 8 4 3 2 1 5 9\n\
                4 5 1 7 6 9 8 3 2\n\
                7 8 3 2 5 4 9 1 6\n\
                2 6 9 3 1 8 5 7 4\n",
            ),
            (
                "4 _ _ _ _ _ _ _ _\n\
                _ _ _ 2 1 8 _ 7 _\n\
                7 _ _ _ 9 _ _ _ 2\n\
                _ _ 6 _ 3 _ 8 _ 4\n\
                1 _ _ _ _ _ _ 2 _\n\
                _ _ 5 _ _ 7 _ _ _\n\
                _ 1 _ _ 6 _ _ _ _\n\
                _ 6 _ _ 8 5 _ _ _\n\
                _ _ 9 _ _ _ _ _ 1\n",
                "4 8 2 7 5 3 9 1 6\n\
                6 9 3 2 1 8 4 7 5\n\
                7 5 1 4 9 6 3 8 2\n\
                2 7 6 5 3 1 8 9 4\n\
                1 3 8 6 4 9 5 2 7\n\
                9 4 5 8 2 7 1 6 3\n\
                5 1 4 9 6 2 7 3 8\n\
                3 6 7 1 8 5 2 4 9\n\
                8 2 9 3 7 4 6 5 1\n",
            ),
            (
                "2 _ _ _ _ 8 _ 4 5\n\
                _ 8 _ 6 _ 7 _ _ _\n\
                9 _ 1 _ _ _ 3 _ _\n\
                7 _ _ 3 _ _ _ _ _\n\
                _ _ 8 _ _ 4 1 7 6\n\
                _ _ 4 1 _ _ 8 _ _\n\
                5 2 _ _ 9 _ _ _ _\n\
                _ 3 _ _ 2 _ _ 5 _\n\
                _ 4 _ _ 6 _ 9 _ 3\n",
                "2 6 3 9 1 8 7 4 5\n\
                4 8 5 6 3 7 2 1 9\n\
                9 7 1 5 4 2 3 6 8\n\
                7 1 2 3 8 6 5 9 4\n\
                3 9 8 2 5 4 1 7 6\n\
                6 5 4 1 7 9 8 3 2\n\
                5 2 6 7 9 3 4 8 1\n\
                8 3 9 4 2 1 6 5 7\n\
                1 4 7 8 6 5 9 2 3\n",
            ),
            (
                "_ _ _ _ 9 6 _ _ _\n\
                _ _ _ 3 _ 2 _ 8 5\n\
                2 _ 4 _ _ 7 _ _ _\n\
                6 _ _ _ 7 _ _ _ _\n\
                9 4 _ _ _ 3 7 _ _\n\
                _ _ _ _ _ _ _ 2 _\n\
                _ 6 9 _ _ _ _ 4 8\n\
                _ _ _ 5 3 _ _ _ 7\n\
                _ 2 _ _ _ _ _ 5 _\n",
                "8 5 3 4 9 6 1 7 2\n\
                7 9 6 3 1 2 4 8 5\n\
                2 1 4 8 5 7 6 3 9\n\
                6 3 2 1 7 5 8 9 4\n\
                9 4 5 2 8 3 7 1 6\n\
                1 7 8 9 6 4 5 2 3\n\
                5 6 9 7 2 1 3 4 8\n\
                4 8 1 5 3 9 2 6 7\n\
                3 2 7 6 4 8 9 5 1\n",
            ),
        ];

        for (input, expected_output) in grids {
            let parsed = read_input(input.as_bytes()).unwrap();
            let solved = solve(parsed);

            assert_eq!(expected_output, print_grid(&solved));
        }
    }
}
