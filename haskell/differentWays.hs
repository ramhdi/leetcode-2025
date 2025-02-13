-- differentWays.hs
-- https://www.hackerrank.com/challenges/different-ways-fp/

import Control.Monad (replicateM_)

modulo :: Integer
modulo = 10 ^ 8 + 7

maxN :: Integer
maxN = 1000

factorials :: [Integer]
factorials = scanl (\acc x -> acc * x `mod` modulo) 1 [1 .. maxN]

modInv :: Integer -> Integer
modInv x = power x (modulo - 2)

power :: Integer -> Integer -> Integer
power _ 0 = 1
power base exp
  | even exp = half * half `mod` modulo
  | otherwise = base * half * half `mod` modulo
  where
    half = power base (exp `div` 2)

f :: Integer -> Integer -> Integer
f n k = (factN * invFactK `mod` modulo) * invFactNK `mod` modulo
  where
    factN = factorials !! fromIntegral n
    factK = factorials !! fromIntegral k
    factNK = factorials !! fromIntegral (n - k)
    invFactK = modInv factK
    invFactNK = modInv factNK

main :: IO ()
main = do
  t <- readLn
  replicateM_ t processCase

processCase :: IO ()
processCase = do
  [n, k] <- fmap (map read . words) getLine
  print (f n k)
