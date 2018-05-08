module Bob (responseFor) where

import Data.Char (isUpper, isAlpha, isSpace)

responseFor :: String -> String
responseFor s | isQuestion s && isShout s = "Calm down, I know what I'm doing!"
              | isQuestion s              = "Sure."
              | isShout s                 = "Whoa, chill out!"
              | isEmpty s                 = "Fine. Be that way!"
              | otherwise                 = "Whatever."
    where
          safeCheck f xs | length xs == 0 = False
                         | otherwise  = f xs

          endsWith c s = let tmp = ((filter (not . isSpace)) . reverse) s in
                             safeCheck ((==c) . head) tmp

          isQuestion = endsWith '?'
          
          isShout s = let tmp = (filter isAlpha) s in
                          safeCheck (all isUpper) tmp

          isEmpty = all isSpace
