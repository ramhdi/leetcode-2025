-- sequenceFullOfColors.hs
-- https://www.hackerrank.com/challenges/sequence-full-of-colors/

import Control.Monad (replicateM_)
import Data.Map qualified as Map

isColorful :: String -> Bool
isColorful = helper (Map.fromList [(c, 0) | c <- "RGYB"])
  where
    helper colorCount [] = (colorCount Map.! 'R' == colorCount Map.! 'G') && (colorCount Map.! 'Y' == colorCount Map.! 'B')
    helper colorCount (x : xs) = do
      let newColorCount = Map.insertWith (+) x 1 colorCount
      let countR = newColorCount Map.! 'R'
      let countG = newColorCount Map.! 'G'
      let countY = newColorCount Map.! 'Y'
      let countB = newColorCount Map.! 'B'

      abs (countR - countG) <= 1 && abs (countY - countB) <= 1 && helper newColorCount xs

processTestCase :: IO ()
processTestCase = do
  colors <- getLine
  let result = isColorful colors
  putStrLn $
    if result
      then "True"
      else "False"

main :: IO ()
main = do
  t <- readLn :: IO Int
  replicateM_ t processTestCase
