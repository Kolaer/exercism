module School (School, add, empty, grade, sorted) where

import qualified Data.Map.Lazy as Map
import Data.List (sort)

data School = School (Map.Map Int [String])

add :: Int -> String -> School -> School
add gradeNum student (School school) = School $ Map.insert gradeNum (student : prevValue) school
                                       where prevValue = case (Map.lookup gradeNum school) of
                                                           Nothing -> []
                                                           Just x  -> x

empty :: School
empty = School Map.empty

grade :: Int -> School -> [String]
grade gradeNum (School school) = case (Map.lookup gradeNum school) of
                                   Nothing -> []
                                   Just x  -> sort $ x

sorted :: School -> [(Int, [String])]
sorted (School school) = fmap (\(gr, studs) -> (gr, sort studs)) $ Map.assocs school
