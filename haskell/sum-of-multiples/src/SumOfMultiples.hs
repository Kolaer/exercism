module SumOfMultiples (sumOfMultiples) where

a `divBy` b = (a `mod` b) == 0

sumOfMultiples :: [Integer] -> Integer -> Integer
sumOfMultiples factors limit = sum $ filter (\x -> any (x `divBy`) factors) [1..limit-1] 
