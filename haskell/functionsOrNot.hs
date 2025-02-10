-- functionsOrNot.hs
-- https://www.hackerrank.com/challenges/functions-or-not/

import Control.Monad (replicateM, replicateM_)
import Data.List (nub)

type Pair = (Int, Int)

isValidFunction :: [Pair] -> Bool
isValidFunction pairs = length (nub xs) == length xs
  where
    xs = map fst pairs

processTestCase :: IO ()
processTestCase = do
  n <- readLn :: IO Int
  pairs <- replicateM n readPair
  putStrLn $ if isValidFunction pairs then "YES" else "NO"
  where
    readPair = do
      [x, y] <- map read . words <$> getLine
      return (x, y)

main :: IO ()
main = do
  t <- readLn :: IO Int
  replicateM_ t processTestCase
