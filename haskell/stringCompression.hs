-- stringCompression.hs
-- https://www.hackerrank.com/challenges/string-compression/

import Data.DList (DList)
import Data.DList qualified as DL

helper :: String -> DList Char -> Char -> Int -> String
helper [] acc lc cc
  | cc > 1 = DL.toList $ acc `DL.append` DL.fromList ([lc] ++ show cc)
  | otherwise = DL.toList $ acc `DL.append` DL.singleton lc
helper (c : xs) acc lc cc
  | c == lc = helper xs acc lc (cc + 1)
  | cc > 1 = helper xs (acc `DL.append` DL.fromList ([lc] ++ show cc)) c 1
  | otherwise = helper xs (acc `DL.append` DL.singleton lc) c 1

compress :: String -> String
compress [] = []
compress (c : xs) = helper xs DL.empty c 1

main :: IO ()
main = do
  s <- getLine
  putStrLn (compress s)
