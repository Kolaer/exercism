module Diamond (diamond) where

import Data.Char
import Data.List

diamond :: Char -> Maybe [String]
diamond char | isAlpha char = Just $ diamond' char
             | otherwise    = Nothing

diamond' :: Char -> [String]
diamond' char = map generate chars
                where chars' = ['A' .. char]
                      chars  = chars' ++ (reverse $ init chars')
                      len    = length chars'

                      generate c = let n = ord c - ord 'A' in
                                     [if i == (len + n) || i == (len - n) then c else ' ' | i <- [1..(2*len - 1)]]
