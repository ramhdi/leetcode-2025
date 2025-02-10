-- polygonArea.hs
-- https://www.hackerrank.com/challenges/lambda-march-compute-the-area-of-a-polygon/

import Control.Monad (replicateM)

type Point = (Double, Double)

area :: [Point] -> Double
area [] = 0
area points@(p : ps) =
  0.5 * abs (sum delta)
  where
    p1 = ps ++ [p]
    x = map fst points
    x1 = map fst p1
    y = map snd points
    y1 = map snd p1
    left = zipWith (*) x y1
    right = zipWith (*) x1 y
    delta = zipWith (-) left right

processTestCase :: IO ()
processTestCase = do
  n <- readLn :: IO Int
  pairs <- replicateM n readPair
  print $ area pairs
  where
    readPair = do
      [x, y] <- map read . words <$> getLine
      return (x, y)

main :: IO ()
main = do
  processTestCase
