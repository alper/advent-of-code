import System.IO

fuelRequired :: Int -> Int
fuelRequired a
    | a <= 0 = 0
    | otherwise = let fuelWeight = max 0 (a `div` 3 - 2)
        in fuelWeight + (fuelRequired fuelWeight)

convertToInteger :: String -> Int
convertToInteger s = read s :: Int

main = do
    handle <- openFile "1-input.txt" ReadMode
    contents <- hGetContents handle
    putStrLn $ show . sum . map fuelRequired . map convertToInteger . lines $ contents