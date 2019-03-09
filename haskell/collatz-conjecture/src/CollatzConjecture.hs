module CollatzConjecture (collatz) where

divBy :: Integral a => a -> a -> Bool
a `divBy` b = (a `mod` b) == 0

collatz :: Integer -> Maybe Integer
collatz x | x <= 0      = Nothing
          | x == 1      = Just 0
          | x `divBy` 2 = (+1) <$> collatz (x `div` 2)
          | otherwise   = (+1) <$> collatz (3 * x + 1)
