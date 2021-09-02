Sudoku Solver
=============

This is a Haskell reimplementation of my Rust sudoku solver (which is in this
Git repository's `master` branch), done as a learning exercise.

To build:

```
stack build
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
>> _ _ 9 _ _ _ _ _ 1" | stack exec sudoku-solver
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
