-- stringReductions.hs
-- https://www.hackerrank.com/challenges/string-reductions/

import Data.Set (Set)
import Data.Set qualified as Set

reduceString :: String -> String
reduceString = helper [] Set.empty
  where
    helper acc _ [] = reverse acc
    helper acc charSet (c : xs)
      | Set.member c charSet = helper acc charSet xs
      | otherwise = do
          let newCharSet = Set.insert c charSet
          helper (c : acc) newCharSet xs

main :: IO ()
main = do
  s <- getLine
  putStrLn (reduceString s)
