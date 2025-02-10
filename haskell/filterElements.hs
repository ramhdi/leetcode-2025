-- filterElements.hs
-- https://www.hackerrank.com/challenges/filter-elements/

import Control.Monad (replicateM_)
import Data.Map qualified as Map
import Data.Set qualified as Set

filterRepeated :: [Int] -> Int -> [Int]
filterRepeated nums k =
  let countOccurrences = foldr (\x -> Map.insertWith (+) x 1) Map.empty nums
      helper seen [] = []
      helper seen (x : xs)
        | Set.member x seen = helper seen xs
        | (countOccurrences Map.! x) >= k =
            x : helper (Set.insert x seen) xs
        | otherwise = helper seen xs
   in helper Set.empty nums

processTestCase :: IO ()
processTestCase = do
  [n, k] <- map read . words <$> getLine :: IO [Int]
  nums <- map read . words <$> getLine :: IO [Int]
  let result = filterRepeated nums k
  putStrLn $
    if null result
      then "-1"
      else unwords (map show result)

main :: IO ()
main = do
  t <- readLn :: IO Int
  replicateM_ t processTestCase
