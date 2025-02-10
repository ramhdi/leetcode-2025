-- listsAndGcd.hs
-- https://www.hackerrank.com/challenges/lists-and-gcd/

import Control.Monad (replicateM)

gcd' :: [Int] -> [Int] -> [Int]
gcd' _ [] = []
gcd' [] _ = []
gcd' a@(pa : na : as) b@(pb : nb : bs)
  | pa == pb = [pa, min na nb] ++ gcd' as bs
  | pa > pb = gcd' a bs
  | pa < pb = gcd' as b

gcdList :: [[Int]] -> [Int]
gcdList [x, y] = gcd' x y
gcdList (x : xs) = gcd' x (gcdList xs)

main :: IO ()
main = do
  n <- readLn :: IO Int
  lists <- replicateM n (map read . words <$> getLine :: IO [Int])
  let result = gcdList lists
  putStrLn $ unwords (map show result)