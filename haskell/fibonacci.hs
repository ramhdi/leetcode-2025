-- fibonacci.hs
-- https://www.hackerrank.com/challenges/functional-programming-warmups-in-recursion---fibonacci-numbers/

fib :: Int -> Int
fib 1 = 0
fib 2 = 1
fib n = fib (n - 1) + fib (n - 2)

main :: IO ()
main = do
  input <- getLine
  print . fib . (read :: String -> Int) $ input
