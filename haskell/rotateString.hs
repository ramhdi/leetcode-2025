-- rotateString.hs
-- https://www.hackerrank.com/challenges/rotate-string/

import Control.Monad (replicateM_)

rotateString :: String -> [String]
rotateString s = helper s 0 []
  where
    helper s i acc
      | i == length s = reverse acc
      | otherwise = do
          let s_rotated = tail s ++ [head s]
          helper s_rotated (i + 1) (s_rotated : acc)

processCase :: IO ()
processCase = do
  s <- getLine
  putStrLn (unwords (rotateString s))

main :: IO ()
main = do
  t <- readLn
  replicateM_ t processCase
