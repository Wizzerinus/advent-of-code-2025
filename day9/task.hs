{-# LANGUAGE OverloadedRecordDot #-}
{-# LANGUAGE LambdaCase #-}

import Control.Monad (when, guard)
import Data.List (unfoldr, sortOn)
import Debug.Trace (trace)
import Data.Maybe (fromJust)
import Control.Exception (throw, Exception)

class Rotatable a where
  invert :: a -> a
  flipR :: a -> a

data Point = Point { x :: Int, y :: Int } deriving (Eq, Show)
point [x, y] = Point x y
data LinePair = LinePair { x01 :: Int, y0 :: Int, x2 :: Int, y12 :: Int } deriving Show

instance Rotatable Point where
  invert (Point x y) = Point (-x) (-y)
  flipR (Point x y) = Point y x

instance Rotatable LinePair where
  invert (LinePair x01 y0 x2 y12) = LinePair (-x01) (-y0) (-x2) (-y12)
  flipR (LinePair x01 y0 x2 y12) = LinePair y12 x2 y0 x01

class Intersectable a where
  outside :: Point -> a
  guardrail :: LinePair -> a -> a
  border :: Point -> LinePair -> [LinePair] -> a

instance Intersectable Bool where
  outside = const False
  guardrail = const not
  border _ _ _ = True

instance Intersectable Int where
  outside = y
  guardrail line _ = line.y12
  border point line xs =
    -- If the point directly above the border is inside the shape, we can go higher,
    -- otherwise the cap is the border's highest point.
    if genericIntersection (Point point.x (1 + max line.y0 line.y12)) xs  -- t ~ Bool
      then genericIntersection point xs                                   -- t ~ Int
      else max line.y0 line.y12

genericIntersection :: Intersectable a => Point -> [LinePair] -> a
-- lines must be sorted by y_{1,2}, in ascending order.
genericIntersection point = \case
  [] -> outside point
  line:xs
    -- If the line lies under our point, we cannot intersect it, so ignore it.
    | line.y12 < point.y && line.y0 < point.y -> genericIntersection point xs
    -- If we lie on the left/right border, we lie inside.
    | line.x01 == point.x && betweenLax point.y line.y0 line.y12 -> border point line xs
    -- If we lie on the top/bottom border, we also lie inside.
    | line.y12 == point.y && betweenLax point.x line.x01 line.x2 -> border point line xs
    -- If we lie beneath the guardrail, we need to toggle the counter.
    | betweenLax point.x line.x01 line.x2 && point.x /= line.x01 && point.y < line.y12 -> guardrail line (genericIntersection point xs)
    -- If we stumble into a side border, move to the side and try again.
    | point.x == line.x01 && point.y < line.y12 && point.y < line.y0 -> genericIntersection (Point (point.x + 1) point.y) (line:xs)
    -- Otherwise, we don't intersect this line.
    | otherwise -> genericIntersection point xs
  where
    betweenLax x l r = x >= min l r && x <= max l r

splitOn :: forall a. Eq a => a -> [a] -> [[a]]
splitOn c = foldr helper [[]]
  where
    helper :: a -> [[a]] -> [[a]]
    helper d (x:xs) = if d == c then [] : x : xs else (d:x) : xs

day2Solver :: [LinePair] -> (Point, Point) -> Bool
day2Solver points (first, second) =
  not (first.x == second.x || first.y == second.y) &&
  genericIntersection first points &&
  genericIntersection second points &&
  let
    maxUp = genericIntersection first points
    maxDown :: Int = -(genericIntersection (invert second) (map invert points))
    maxRight = genericIntersection (flipR first) (map flipR points)
    maxLeft = -(genericIntersection (invert $ flipR second) (map (invert . flipR) points))
  in maxRight >= second.x && maxLeft <= first.x && maxUp >= second.y && maxLeft <= first.y

mkLines :: [Point] -> [LinePair]
mkLines (x:y:z:xs) = LinePair x.x x.y z.x z.y : mkLines (z:xs)
mkLines [_, _] = []

cyclicShift :: Int -> [a] -> [a]
cyclicShift n xs = xs ++ take n xs

data Err = TwoFirstXsMustMatch deriving Show
instance Exception Err

main :: IO ()
main = do
  rows <- fmap (point . fmap read . splitOn ',') . lines <$> readFile "./input9.txt"
  let r1:r2:_ = rows
  when (r1.x /= r2.x) $ throw TwoFirstXsMustMatch  -- this could be processed in a nicer way, but I already spent too long on this problem
  let lines = sortOn y12 $ mkLines $ cyclicShift 2 rows
  let
    solver filter = do
      Point x1 y1 <- rows
      Point x2 y2 <- rows
      let (x1', x2') = (min x1 x2, max x1 x2)
      let (y1', y2') = (min y1 y2, max y1 y2)
      pure $ if filter (Point x1' y1', Point x2' y2') then (x2' - x1' + 1) * (y2' - y1' + 1) else 0
  putStrLn $ "Day 1: " ++ show (maximum $ solver (const True))
  putStrLn $ "Day 2: " ++ show (maximum $ solver (day2Solver lines))
