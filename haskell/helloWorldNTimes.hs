-- helloWorldNTimes.hs
-- https://www.hackerrank.com/challenges/fp-hello-world-n-times/

repeatString :: String -> Int -> IO ()
repeatString str n
  | n <= 0 = return ()
  | otherwise = do
      putStrLn str
      repeatString str (n - 1)

main :: IO ()
main = do
  n <- readLn :: IO Int
  repeatString "Hello World" n