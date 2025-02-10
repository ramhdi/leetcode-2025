-- superDigit.hs
-- https://www.hackerrank.com/challenges/super-digit/

digits :: Integer -> [Integer]
digits n = map (read . pure) (show n)

superDigit :: Integer -> Integer
superDigit num = helper (digits num)
  where
    helper d
      | length d == 1 = head d
      | otherwise = superDigit (sum d)

main :: IO ()
main = do
  input <- getLine
  let [num, times] = map read $ words input :: [Integer]
  let initialSum = sum (digits num)
  let p = initialSum * times
  print (superDigit p)
