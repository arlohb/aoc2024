findInLines :: String -> Int
findInLines [] = 0
findInLines ('X' : 'M' : 'A' : 'S' : xs) = 1 + findInLines xs
findInLines (x : xs) = findInLines xs

-- e.g. [ [1,2,3], [4,5,6] ] [ 8, 9 ] -> [ [1,2,3,8], [4,5,6,9] ]
zipMultiple' :: [[a]] -> [a] -> [[a]]
zipMultiple' = zipWith $ \acc next -> acc ++ [next]

-- e.g. [ [1,2,3], [4,5,6], [7,8,9] ] -> [ [1,4,7], [2,5,8], [3,6,9] ]
zipMultiple :: [[a]] -> [[a]]
zipMultiple [] = []
zipMultiple (x:xs) = foldl zipMultiple' (map (: []) x) xs

verticalLines :: [String] -> [String]
verticalLines = zipMultiple

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

diagonalLines :: [String] -> [String]
diagonalLines xs = verticalLines (zipWith shiftLeftN (map padLine xs) [0..])
    ++ verticalLines (zipWith shiftRightN (map padLine xs) [0..])

appendReverse :: [[a]] -> [[a]]
appendReverse = concatMap (\xs -> [xs, reverse xs])

combinations :: [String] -> [String]
combinations lines = appendReverse $ lines
    ++ verticalLines lines
    ++ diagonalLines lines

count :: [String] -> Int
count lines = foldl (\acc x -> acc + findInLines x) 0 $ combinations lines

main :: IO ()
main = do
    input <- readFile "input.txt"
    print $ count (lines input)

