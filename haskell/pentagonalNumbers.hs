-- pentagonalNumbers.hs
-- https://www.hackerrank.com/challenges/pentagonal-numbers/

import Control.Monad (replicateM)
import Data.Array

pentagonalArray :: Array Int Int
pentagonalArray = listArray (1, 100000) [(n * (3 * n - 1)) `div` 2 | n <- [1 .. 100000]]

main :: IO ()
main = do
  t <- readLn
  queries <- replicateM t readLn
  mapM_ (print . (pentagonalArray !)) queries
