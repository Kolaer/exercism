module Pangram (isPangram) where

import Data.Char
import Data.List

isPangram :: String -> Bool
isPangram = (==['a'..'z']) . nub . sort . letters

letters :: String -> String
letters = filter isAlpha . map toLower
