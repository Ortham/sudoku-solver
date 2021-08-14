Sudoku Solver
=============

This is a clean room implementation (meaning I have no idea what I'm doing) of a
sudoku solver. The application reads a sudoku grid from stdin and prints the
solved grid to stdout.

The input grid must be laid out as nine lines of nine whitespace-separated
symbols: anything that is not a non-zero number or whitespace will be
interpreted as an empty cell in the grid that needs solving. Once nine lines
have been read, the solver stops reading input and starts calculating the
solution.

The solver logic is not complicated or particularly clever: it just loops over
each cell in the grid and checks what the possible values are given other values
present in the same row, column and 3x3 box. For each possible value, it creates
a copy of the grid with that value substituted in, and then attempts to solve
that grid. If a grid turns out to be unsolvable (i.e. it finds that a cell has
no possible values), it discards that grid. Assuming the sudoku puzzle is
actually solveable, it ends up with a grid with all cells filled, and stops
there.

I've attempted to avoid allocating on the heap inside the solver's hot loop,
which is why there are a lot of fixed-size arrays used. This does seem to have
an impact on performance, though my measurements were very unrigorous and the
solver was already fast enough for my needs anyway.

To build:

```
cargo build --release
```

Here's some example input and output, using underscores to indicate empty cells:

```powershell
PS C:\sudoku-solver> echo "4 _ _ _ _ _ _ _ _
>> _ _ _ 2 1 8 _ 7 _
>> 7 _ _ _ 9 _ _ _ 2
>> _ _ 6 _ 3 _ 8 _ 4
>> 1 _ _ _ _ _ _ 2 _
>> _ _ 5 _ _ 7 _ _ _
>> _ 1 _ _ 6 _ _ _ _
>> _ 6 _ _ 8 5 _ _ _
>> _ _ 9 _ _ _ _ _ 1" | .\target\release\sudoku-solver.exe
Solved sudoku in 22158 loops!
Took 2 ms to solve
Solved values:
4 8 2 7 5 3 9 1 6
6 9 3 2 1 8 4 7 5
7 5 1 4 9 6 3 8 2
2 7 6 5 3 1 8 9 4
1 3 8 6 4 9 5 2 7
9 4 5 8 2 7 1 6 3
5 1 4 9 6 2 7 3 8
3 6 7 1 8 5 2 4 9
8 2 9 3 7 4 6 5 1
```
