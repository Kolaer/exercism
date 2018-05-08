module CollatzConjecture (collatz) where

a `divBy` b = (a `mod` b) == 0

collatz :: Integer -> Maybe Integer
collatz x | x <= 0         = Nothing
          | otherwise      = Just (collatz' x)
    where collatz' x | x == 1      = 0
                     | x `divBy` 2 = 1 + collatz' (x `div` 2)
                     | otherwise   = 1 + collatz' (3 * x + 1)
