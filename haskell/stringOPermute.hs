-- stringOPermute.hs
-- https://www.hackerrank.com/challenges/string-o-permute/

import Control.Monad (replicateM)

swap :: String -> String
swap [] = []
swap (e : o : xs) = o : e : swap xs

main :: IO ()
main = do
  t <- readLn :: IO Int
  inputs <- replicateM t getLine
  let results = map swap inputs
  mapM_ putStrLn results
