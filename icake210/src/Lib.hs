module Lib where
import Data.Bits

newtype XOR a = XOR { getXOR :: a }

instance Bits a => Semigroup (XOR a) where
  XOR a <> XOR b = XOR (a `xor` b)

findUnfulfilledDelivery :: (Foldable f, Bits a) => f a -> Maybe a
findUnfulfilledDelivery input = getXOR <$> foldMap (Just . XOR) input
