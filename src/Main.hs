module Main where

import Data.Char (isSpace)
import Data.Maybe (isJust, isNothing, fromMaybe, listToMaybe)
import Data.List (find, findIndex)
import Debug.Trace

data CellValue = One | Two | Three | Four | Five | Six | Seven | Eight | Nine deriving (Eq, Show)

type Cell = Maybe CellValue

type Row = [Cell]

type Grid = [Row]

newtype Error = Error String

allCellValues = [One, Two, Three, Four, Five, Six, Seven, Eight, Nine]

printValue :: CellValue -> Char
printValue One   = '1'
printValue Two   = '2'
printValue Three = '3'
printValue Four  = '4'
printValue Five  = '5'
printValue Six   = '6'
printValue Seven = '7'
printValue Eight = '8'
printValue Nine  = '9'

main :: IO ()
main = do
    gridOrError <- readInput []
    case gridOrError of
        Right inputGrid ->
            case solve inputGrid of
                Just solvedGrid -> putStrLn ("Solved values:\n" ++ printGrid solvedGrid)
                Nothing -> putStrLn "Error: could not solve sudoku!"
        Left (Error message) -> putStrLn ("Error: " ++ message)

readInput :: Grid -> IO (Either Error Grid)
readInput a = case length a of
        9 -> return (Right a)
        _ -> do
            line <- getLine
            case checkRow $ strToRow line of
                Right row -> readInput $ a ++ [row]
                Left error -> return $ Left error

printCell :: Cell -> Char
printCell (Just c) = printValue c
printCell Nothing = '_'

printRow :: Row -> String
printRow (c:cs) = printCell c : ' ' : printRow cs
printRow [] = ""

printGrid :: Grid -> String
printGrid rows = unlines $ map printRow rows

strToRow :: String -> Row
strToRow [] = []
strToRow ('1':cs) = Just One : strToRow cs
strToRow ('2':cs) = Just Two : strToRow cs
strToRow ('3':cs) = Just Three : strToRow cs
strToRow ('4':cs) = Just Four : strToRow cs
strToRow ('5':cs) = Just Five : strToRow cs
strToRow ('6':cs) = Just Six : strToRow cs
strToRow ('7':cs) = Just Seven : strToRow cs
strToRow ('8':cs) = Just Eight : strToRow cs
strToRow ('9':cs) = Just Nine : strToRow cs
strToRow (c:cs) = if isSpace c then strToRow cs else Nothing : strToRow cs

checkRow :: Row -> Either Error Row
checkRow list
    | length list > 9 = Left $ Error "The row has too many columns!"
    | length list < 9 = Left $ Error "The row has too few columns!"
    | length list == 9 = Right list

isSolved :: Grid -> Bool
isSolved = all (all isJust)

valueExistsInRow :: Grid -> Int -> CellValue -> Bool
valueExistsInRow grid rowIndex = \value -> any (cellHasValue value) row
    where row = grid !! rowIndex

valueExistsInColumn :: Grid -> Int -> CellValue -> Bool
valueExistsInColumn grid columnIndex = \value -> any (cellHasValue value) column
    where column = getCellsInColumn grid columnIndex

getCellsInColumn :: Grid -> Int -> [Cell]
getCellsInColumn (row:rows) columnIndex = row !! columnIndex : getCellsInColumn rows columnIndex
getCellsInColumn [] _ = []

valueExistsInBox :: Grid -> Int -> Int -> CellValue -> Bool
valueExistsInBox grid rowIndex columnIndex = \value -> any (cellHasValue value) boxCells
    where boxCells = getCellsInBox grid rowIndex columnIndex

getCellsInBox :: Grid -> Int -> Int -> [Cell]
getCellsInBox grid rowIndex columnIndex = concatMap (take 3 . drop ignoredColumnCount) rows
    where
        ignoredRowCount = getBoxStartingIndex rowIndex
        ignoredColumnCount = getBoxStartingIndex columnIndex
        rows = take 3 $ drop ignoredRowCount grid

getBoxStartingIndex :: Int -> Int
getBoxStartingIndex index | index < 3 = 0
getBoxStartingIndex index | index < 6 = 3
getBoxStartingIndex index = 6

valueExists :: Grid -> Int -> Int -> CellValue -> Bool
valueExists grid rowIndex columnIndex value =
    valueExistsInRow grid rowIndex value
    || valueExistsInColumn grid columnIndex value
    || valueExistsInBox grid rowIndex columnIndex value

cellHasValue :: CellValue -> Cell -> Bool
cellHasValue value (Just v) | v == value = True
cellHasValue _ _ = False

getPossibleValues :: Grid -> Int -> Int -> [CellValue]
getPossibleValues grid rowIndex columnIndex =
    case (grid !! rowIndex) !! columnIndex of
        Just value -> [value]
        Nothing -> filter (not . valueExists grid rowIndex columnIndex) allCellValues

replaceCell :: Grid -> Int -> Int -> CellValue -> Grid
replaceCell grid rowIndex columnIndex value = beforeRows ++ [replacedRow] ++ tail rows
    where
        (beforeRows, rows) = splitAt rowIndex grid
        (beforeColumns, columns) = splitAt columnIndex (head rows)
        replacedRow = beforeColumns ++ [Just value] ++ tail columns

getEmptyCellIndices :: Grid -> [(Int, Int)]
getEmptyCellIndices grid = [
        (rowIndex, columnIndex) |
        (rowIndex, row) <- zip [0..] grid,
        (columnIndex, cell) <- zip [0..] row,
        isNothing cell
    ]

getFirstEmptyCellIndices :: Grid -> Maybe (Int, Int)
getFirstEmptyCellIndices grid = listToMaybe $ getEmptyCellIndices grid

-- If no empty cells, we don't need any further permutations, just return this grid.
-- If there is an empty cell with no possible values, return no grids
-- Otherwise, return a grid for each possible value of that empty cell.
getPermutations :: Grid -> [Grid]
getPermutations grid = maybe [grid] mapper cellIndices
    where
        cellIndices = getFirstEmptyCellIndices grid
        mapper = \(rowIndex, columnIndex) -> map (replaceCell grid rowIndex columnIndex) (getPossibleValues grid rowIndex columnIndex)

getRecursivePermutations :: Grid -> [Grid]
getRecursivePermutations grid = case getPermutations grid of
    [grid] | isSolved grid -> [grid]
    grids -> concatMap getRecursivePermutations grids

solve :: Grid -> Maybe Grid
solve grid = find isSolved (getRecursivePermutations grid)
