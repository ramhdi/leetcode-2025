-- helloWorld.hs
-- https://www.hackerrank.com/challenges/fp-hello-world/

helloWorld :: IO ()
helloWorld = putStrLn "Hello World"

main :: IO ()
main = do
  helloWorld