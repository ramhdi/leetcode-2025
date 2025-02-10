-- pascalTriangle.hs
-- https://www.hackerrank.com/challenges/pascals-triangle/

nextRow :: [Integer] -> [Integer]
nextRow xs = zipWith (+) (0 : xs) (xs ++ [0])

pascal :: Int -> [[Integer]]
pascal n = take n $ iterate nextRow [1]

printRow :: [Integer] -> String
printRow xs = unwords (map show xs)

printTriangle :: [[Integer]] -> IO ()
printTriangle = mapM_ (putStrLn . printRow)

main :: IO ()
main = do
  n <- readLn
  printTriangle $ pascal n
