module Isogram (isIsogram) where

import Data.List (nub)
import Data.Char (toLower, isAlpha)

isIsogram :: String -> Bool
isIsogram = (\x -> x == nub x) . map toLower . filter isAlpha
