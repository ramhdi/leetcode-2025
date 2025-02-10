-- stringMingling.hs
-- https://www.hackerrank.com/challenges/string-mingling/

mingle :: String -> String -> String
mingle [] [] = []
mingle (p : ps) (q : qs) = p : q : mingle ps qs

main :: IO ()
main = do
  p <- getLine
  q <- getLine
  putStrLn (mingle p q)
