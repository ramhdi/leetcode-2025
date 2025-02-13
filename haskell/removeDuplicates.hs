-- removeDuplicates.hs
-- https://www.hackerrank.com/challenges/remove-duplicates/

import Data.Set qualified as Set

removeDuplicates :: String -> String
removeDuplicates s = helper s Set.empty []
  where
    helper [] _ acc = reverse acc
    helper (c : cs) charSet acc
      | Set.member c charSet = helper cs charSet acc
      | otherwise = helper cs (Set.insert c charSet) (c : acc)

main :: IO ()
main = do
  s <- getLine
  putStrLn (removeDuplicates s)
