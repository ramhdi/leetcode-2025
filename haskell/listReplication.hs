-- listReplication.hs
-- https://www.hackerrank.com/challenges/fp-list-replication/

f :: Int -> [Int] -> [Int]
f n arr = [num | num <- arr, _ <- [1 .. n]]

main :: IO ()
main =
  getContents
    >>= mapM_ print . (\(n : arr) -> f n arr) . map read . words