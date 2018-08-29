module Lib
  ( LocatedRect
  , newLocatedRect
  , zeroRectangle
  , leftX
  , bottomY
  , width
  , height
  , overlap
  ) where
import Lens.Micro

data Point
  = Point
  { pointX :: Integer
  , pointY :: Integer
  } deriving (Eq, Ord, Show)

_pointX :: Lens' Point Integer
_pointX = lens pointX $ \v p -> v { pointX = p }

_pointY :: Lens' Point Integer
_pointY = lens pointY $ \v p -> v { pointY = p }

data Vector
  = Vector
  { sizeX :: Integer
  , sizeY :: Integer
  } deriving (Eq, Ord, Show)

_sizeX :: Lens' Vector Integer
_sizeX = lens sizeX $ \v p -> v { sizeX = p }

_sizeY :: Lens' Vector Integer
_sizeY = lens sizeY $ \v p -> v { sizeY = p }

data LocatedRect
  = LocatedRect
  { location :: Point
  , size :: Vector
  } deriving (Show, Eq, Ord)

_location :: Lens' LocatedRect Point
_location = lens location $ \v l -> v { location = l }

_size :: Lens' LocatedRect Vector
_size = lens size $ \v vec -> v { size = vec }

leftX :: Lens' LocatedRect Integer
leftX = _location . _pointX

bottomY :: Lens' LocatedRect Integer
bottomY = _location . _pointY

width :: Lens' LocatedRect Integer
width = _size . _sizeX

height :: Lens' LocatedRect Integer
height = _size . _sizeY

zeroRectangle :: LocatedRect
zeroRectangle = newLocatedRect 0 0 0 0

newLocatedRect :: Integer -> Integer -> Integer -> Integer -> LocatedRect
newLocatedRect leftX' bottomY' width' height' = LocatedRect (Point leftX' bottomY') (Vector width' height')

overlap :: LocatedRect -> LocatedRect -> Maybe LocatedRect
overlap a@(LocatedRect pA _) b@(LocatedRect pB _) =
  let newLoc = maxPoint pA pB
      newTop = minPoint (topCorner a) (topCorner b)
   in pointsToRect newLoc newTop

pointwiseZip :: (Integer -> Integer -> Integer) -> Point -> Point -> Point
pointwiseZip f a b =
  Point { pointX = f (pointX a) (pointX b)
        , pointY = f (pointY a) (pointY b)
        }

minPoint :: Point -> Point -> Point
minPoint = pointwiseZip min

maxPoint :: Point -> Point -> Point
maxPoint = pointwiseZip max

topCorner :: LocatedRect -> Point
topCorner LocatedRect {..} = pointAddSize location size

pointAddSize :: Point -> Vector -> Point
pointAddSize Point {..} Vector {..} =
  Point { pointX = pointX + sizeX
        , pointY = pointY + sizeY
        }

pointDifference :: Point -> Point -> Maybe Vector
pointDifference (Point xA yA) (Point xB yB) =
  case (xB - xA, yB - yA) of
    (x, y) | x > 0 && y > 0 -> Just $ Vector x y
           | otherwise -> Nothing

pointsToRect :: Point -> Point -> Maybe LocatedRect
pointsToRect p q = do
  rect <- pointDifference p q
  return $ LocatedRect p rect
