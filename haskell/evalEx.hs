-- evalEx.hs
-- https://www.hackerrank.com/challenges/eval-ex/

import Control.Monad (forM_)

f :: Double -> Double
f x = helper x 1 1.0 1.0 1
  where
    helper x term sum i count
      | count == 10 = sum
      | otherwise =
          helper
            x
            (term * x / i)
            (sum + term * x / i)
            (i + 1)
            (count + 1)

main :: IO ()
main = do
  n <- readLn :: IO Int

  forM_ [1 .. n] $ \n_itr -> do
    x <- readLn :: IO Double
    print (f x)
