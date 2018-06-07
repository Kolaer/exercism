module Poker (bestHands) where

import           Data.Char
import           Data.Function
import           Data.List
import           Data.Maybe

bestHands :: [String] -> Maybe [String]
bestHands strings = case hands of
                      Just h  -> Just $ pickBest h
                      Nothing -> Nothing
  where hands = mapM parseCards strings

        pickBest h = let values = map handValue h
                         maxVal = maximum values
                         ids    = map snd $ filter (\(v,_) -> v == maxVal) $ zip values [0..] in
                       map (\i -> strings !! i) ids


data Value = Two
           | Three
           | Four
           | Five
           | Six
           | Seven
           | Eight
           | Nine
           | Ten
           | Jack
           | Queen
           | King
           | Ace deriving (Show, Read, Eq, Ord, Bounded, Enum)


data Suit = Spades
          | Clubs
          | Hearts
          | Diamonds deriving (Show, Read, Eq, Ord, Bounded, Enum)


data Card = Card Value Suit deriving (Show, Read)

instance Eq Card where
  (==) c1 c2 = compareCards c1 c2 == EQ

instance Ord Card where
  compare = compareCards


cardValue (Card value _) = value
cardSuit  (Card _ suit)  = suit

compareCards = compare `on` cardValue


parseCard :: String -> Maybe Card
parseCard s | suit == 'S' = parseVal val >>= (\v -> Just $ Card v Spades)
            | suit == 'C' = parseVal val >>= (\v -> Just $ Card v Clubs)
            | suit == 'H' = parseVal val >>= (\v -> Just $ Card v Hearts)
            | suit == 'D' = parseVal val >>= (\v -> Just $ Card v Diamonds)
            | otherwise   = Nothing
  where suit = last s
        val  = init s

        parseVal x | all isDigit x && 2 <= read x && read x <= 10 = Just $ toEnum $ read x - 2
                   | x == "J"      = Just Jack
                   | x == "Q"      = Just Queen
                   | x == "K"      = Just King
                   | x == "A"      = Just Ace
                   | otherwise     = Nothing


parseCards :: String -> Maybe [Card]
parseCards = mapM parseCard . words


checkHighCard xs = (0, sortBy (flip compare) (concat xs))


checkPair ([x,_]:xs) = Just (1, x : snd (checkHighCard xs))
checkPair _          = Nothing


checkTwoPair ([x,_]:[y,_]:xs) = Just (2, max x y : min x y : snd (checkHighCard xs))
checkTwoPair _                = Nothing


checkThreeOfKind ([x,_,_]:xs) = Just (3, x : snd (checkHighCard xs))
checkThreeOfKind _            = Nothing


checkStraight xs | flag      = Just (4, [if aceFirst then last $ init sorted else last sorted])
                 | otherwise = Nothing
  where cards  = concat xs
        sorted = sort cards
        values = map cardValue sorted

        aceFirst = head values == Two && last values == Ace
        vals     = if aceFirst then init values else values -- if ace is first drop it
        flag     = all (uncurry (==)) $ zip vals [head vals..]


checkFlush xs | flag      = Just (5, sortBy (flip compare) cards)
              | otherwise = Nothing
  where cards = concat xs

        firstSuit = cardSuit $ head cards
        restSuits = map cardSuit $ tail cards

        flag  = all (== firstSuit) restSuits


checkFullHouse ([x,_,_]:[y,_]:_) = Just (6, [x, y])
checkFullHouse _                 = Nothing


checkFourOfKind ([x,_,_,_]:xs) = Just (7, x : snd (checkHighCard xs))
checkFourOfKind _              = Nothing


checkStraightFlush xs | flag      = Just (8, [maximum cards])
                      | otherwise = Nothing
  where cards = concat xs
        flag  = isJust (checkFlush xs) && isJust (checkStraight xs)


groupHand hand = reverse $ sortOn length $ groupBy ((==) `on` cardValue) $ sort hand

handValue :: [Card] -> (Int, [Card])
handValue hand | not (null combRes) = fromJust $ head combRes
               | otherwise          = checkHighCard grouped
  where grouped = groupHand hand

        combinations = [checkStraightFlush, checkFourOfKind, checkFullHouse, checkFlush, checkStraight, checkThreeOfKind, checkTwoPair, checkPair]

        combRes = filter isJust $ map (\f -> f grouped) combinations
