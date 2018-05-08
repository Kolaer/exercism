module PerfectNumbers (classify, Classification(..)) where

data Classification = Deficient | Perfect | Abundant deriving (Eq, Show)

a `divBy` b = (a `mod` b) == 0

classify :: Int -> Maybe Classification
classify x | x <= 0 = Nothing
           | otherwise = case compare x (helper x) of
                             LT -> Just Abundant
                             EQ -> Just Perfect
                             GT -> Just Deficient
                          where helper x = sum $ filter (x `divBy` ) [1..x-1]
