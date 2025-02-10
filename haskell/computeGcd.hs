-- computeGcd.hs
-- https://www.hackerrank.com/challenges/functional-programming-warmups-in-recursion---gcd/

module Main where

gcd' :: (Integral a) => a -> a -> a
gcd' n m
  | n == m = n
  | n > m = gcd' (n - m) m
  | otherwise = gcd m n

main :: IO ()
main = do
  input <- getLine
  print . uncurry gcd' . listToTuple . convertToInt . words $ input
  where
    listToTuple (x : xs : _) = (x, xs)
    convertToInt = map (read :: String -> Int)
