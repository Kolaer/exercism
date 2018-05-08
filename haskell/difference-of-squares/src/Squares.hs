module Squares (difference, squareOfSums, sumOfSquares) where

difference :: Integral a => a -> a
difference n = (n * (n + 1) * (3*n + 2) * (n - 1)) `div` 12

squareOfSums :: Integral a => a -> a
squareOfSums n = (n^2 * (n + 1)^2) `div` 4 

sumOfSquares :: Integral a => a -> a
sumOfSquares n = (n * (n + 1) * (2*n + 1)) `div` 6
