import Data.List (transpose)

findInLines :: String -> Int
findInLines [] = 0
findInLines ('X' : 'M' : 'A' : 'S' : xs) = 1 + findInLines xs
findInLines (x : xs) = findInLines xs

repeatF :: (a -> a) -> Int -> a -> a
repeatF f 0 x = x
repeatF f n x = repeatF f (n - 1) (f x)

shiftLeft :: String -> String
shiftLeft (x:xs) = xs ++ [x]

shiftLeftN :: String -> Int -> String
shiftLeftN xs n = repeatF shiftLeft n xs

shiftRight :: String -> String
shiftRight xs = last xs : init xs

shiftRightN :: String -> Int -> String
shiftRightN xs n = repeatF shiftRight n xs

padLine :: String -> String
padLine s = '.' : s ++ "."

mapWithIndex :: (a -> Int -> b) -> [a] -> [b]
mapWithIndex f xs = zipWith f xs [0..]

diagonalLines :: [String] -> [String]
diagonalLines xs = let
        padded = map padLine xs
    in transpose (mapWithIndex shiftLeftN padded)
    ++ transpose (mapWithIndex shiftRightN padded)

appendReverse :: [[a]] -> [[a]]
appendReverse = concatMap (\xs -> [xs, reverse xs])

combinations :: [String] -> [String]
combinations lines = appendReverse $ lines
    ++ transpose lines
    ++ diagonalLines lines

count :: [String] -> Int
count lines = foldl (\acc x -> acc + findInLines x) 0 $ combinations lines

main :: IO ()
main = do
    input <- readFile "input.txt"
    print $ count (lines input)

