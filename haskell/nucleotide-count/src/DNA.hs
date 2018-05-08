module DNA (nucleotideCounts) where

import Data.Map (Map, (!?), insert, fromList)
import Control.Monad (foldM)

nucleotideCounts :: String -> Either String (Map Char Int)
nucleotideCounts xs = foldM helper empty xs
                      where 
                            helper acc c = case acc !? c of
                                                Nothing -> Left "Wrong character."
                                                Just n  -> Right $ insert c (n + 1) acc

                            empty = fromList [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
