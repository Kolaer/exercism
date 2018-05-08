module DNA (toRNA) where

toRNA :: String -> Maybe String
toRNA xs = sequence $ map convert xs
    where convert 'G' = Just 'C'
          convert 'C' = Just 'G'
          convert 'T' = Just 'A'
          convert 'A' = Just 'U'
          convert _   = Nothing
