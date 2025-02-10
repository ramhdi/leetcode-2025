-- sumsOfPower.hs
-- https://www.hackerrank.com/challenges/functional-programming-the-sums-of-powers/

findCombinations :: Int -> [Int] -> Int
findCombinations 0 _ = 1
findCombinations _ [] = 0
findCombinations target (x : xs)
  | target < 0 = 0
  | otherwise = withX + withoutX
  where
    withX = findCombinations (target - x) xs
    withoutX = findCombinations target xs

ways :: Int -> Int -> Int
ways x n = do
  let powerSeq = takeWhile (<= x) [i ^ n | i <- [1 ..]]
  findCombinations x powerSeq

main :: IO ()
main = do
  x <- readLn :: IO Int
  n <- readLn :: IO Int
  print (ways x n)
