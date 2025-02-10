-- polygonPerimeter.hs
-- https://www.hackerrank.com/challenges/lambda-march-compute-the-perimeter-of-a-polygon/

import Control.Monad (replicateM)

type Point = (Double, Double)

distance :: Point -> Point -> Double
distance (x1, y1) (x2, y2) = sqrt ((x2 - x1) ^ 2 + (y2 - y1) ^ 2)

perimeter :: [Point] -> Double
perimeter [] = 0
perimeter points@(p : ps) = sum $ zipWith distance points (ps ++ [p])

processTestCase :: IO ()
processTestCase = do
  n <- readLn :: IO Int
  pairs <- replicateM n readPair
  print $ perimeter pairs
  where
    readPair = do
      [x, y] <- map read . words <$> getLine
      return (x, y)

main :: IO ()
main = do
  processTestCase
