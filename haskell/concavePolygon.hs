-- concavePolygon.hs
-- https://www.hackerrank.com/challenges/lambda-march-concave-polygon/

import Control.Monad (replicateM)
import Data.Function (on)
import Data.List (sortBy)

type Point = (Double, Double)

type Vector = (Double, Double)

-- Calculate centroid of points
centroid :: [Point] -> Point
centroid ps =
  let (sumX, sumY) = foldr (\(x, y) (ax, ay) -> (ax + x, ay + y)) (0, 0) ps
      n = fromIntegral $ length ps
   in (sumX / n, sumY / n)

-- Calculate angle between point and centroid
angle :: Point -> Point -> Double
angle (cx, cy) (x, y) = atan2 (y - cy) (x - cx)

-- Sort points counterclockwise around centroid
sortPoints :: [Point] -> [Point]
sortPoints ps =
  let c = centroid ps
   in sortBy (compare `on` angle c) ps

-- Vector between two points
vector :: Point -> Point -> Vector
vector (x1, y1) (x2, y2) = (x2 - x1, y2 - y1)

-- Cross product of two vectors
cross :: Vector -> Vector -> Double
cross (x1, y1) (x2, y2) = x1 * y2 - y1 * x2

-- Check if three points make a right turn (indicates concavity when moving counterclockwise)
isRightTurn :: Point -> Point -> Point -> Bool
isRightTurn p1 p2 p3 =
  let v1 = vector p1 p2
      v2 = vector p2 p3
   in cross v1 v2 < 0

-- Create sliding windows of three points, wrapping around
triples :: [Point] -> [(Point, Point, Point)]
triples ps =
  let n = length ps
      ps' = ps ++ take 2 ps -- Add first two points to end
   in [(ps' !! i, ps' !! (i + 1), ps' !! (i + 2)) | i <- [0 .. n - 1]]

-- Check if polygon is concave
isConcave :: [Point] -> Bool
isConcave ps =
  let ordered = sortPoints ps
      turns = triples ordered
   in any (\(p1, p2, p3) -> isRightTurn p1 p2 p3) turns

-- Parse point from string
parsePoint :: String -> Point
parsePoint s =
  let [x, y] = map read $ words s
   in (fromIntegral x, fromIntegral y)

-- Main IO function
main :: IO ()
main = do
  n <- readLn :: IO Int
  points <- replicateM n (fmap parsePoint getLine)
  putStrLn $ if isConcave points then "YES" else "NO"
