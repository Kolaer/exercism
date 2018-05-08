module Acronym (abbreviate) where

import Data.Char (toUpper, isUpper)

abbreviate :: String -> String
abbreviate = concatMap helper . words . map change 
             where
                   helper (c:cs) = toUpper c : filter isUpper (dropWhile isUpper cs)

                   change '-' = ' '
                   change c   = c
