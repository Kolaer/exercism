module LeapYear (isLeapYear) where

divBy a b = a `mod` b == 0

isLeapYear :: Integer -> Bool
isLeapYear year | year `divBy` 400 = True
                | year `divBy` 100 = False
                | year `divBy` 4   = True
                | otherwise        = False 
