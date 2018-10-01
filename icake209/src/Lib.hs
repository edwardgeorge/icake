module Lib where

fib :: Int -> Int
fib nth
  | nth < 1 = 0
  | nth == 1 = 1
  | otherwise = go (nth - 2) 0 1
  where
    go 0 x y = x + y
    go n x y = go (n - 1) y $! x + y
