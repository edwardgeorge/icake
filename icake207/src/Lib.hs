module Lib where
import           Control.Applicative
import           Control.Monad
import           Control.Monad.Trans.Class
import           Control.Monad.Trans.State.Lazy
import           Data.Char hiding (Space)
import           Data.Foldable
import           Data.Functor
import           Data.HashMap.Strict (HashMap)
import qualified Data.HashMap.Strict as Map
import           Data.Maybe
import           Data.Text (Text)
import qualified Data.Text as Text

-- Our dict-like type for holding the word-count
type Dict = HashMap Text Int

-- -------------------------------------------------
-- Main API
-- -------------------------------------------------

-- | Build a word-count 'Dict' from a 'String'
dictFromString :: String -> Dict
dictFromString = flip addSentence mempty

-- | Add a sentence to an existing word-count 'Dict'
--
-- We lex the input String into a stream of tokens which
-- is parsed with our 'sentence' parser returning a stream
-- of lower-case words which are added individually to the
-- provided 'Dict'.
addSentence :: String -> Dict -> Dict
addSentence t d = foldl' (flip addWord) d words'
  where
    words' = fromMaybe (error "Parse error") . evalStateT sentence $ lexer t

-- | Add a single word to the word-count 'Dict'
addWord :: Text -> Dict -> Dict
addWord t = Map.insertWith (+) t 1

-- -------------------------------------------------
-- Lexer
--
-- We use a lexer to convert the input string into a
-- stream of 'Token's.
-- This will form the input for our parser.
--
-- The tokens are a contiguous word fragment (in
-- lower-case), punctuation, or space.
-- -------------------------------------------------

data Token
  = Word String
  | Punctuation Char
  | Space
  deriving (Eq, Ord, Show)

elimToken :: (String -> a) -> (Char -> a) -> a -> Token -> a
elimToken f g x t = case t of
  Word w -> f w
  Punctuation c -> g c
  Space -> x

-- | Convert a 'String' into a stream of 'Token's
-- Each word is lower-cased here.
lexer :: String -> [Token]
lexer = foldr go []
  where
    go c r
      | isPunctuation c = Punctuation c : r
      | isSpace c = Space : r
      | otherwise =
        case r of
          (Word w:r') -> Word (toLower c : w) : r'
          _ -> Word [toLower c] : r

-- -------------------------------------------------
-- Parser
--
-- We create a Parser to convert the stream of
-- tokens from the lexer into a stream of words.
--
-- The simple type of a parser is:
-- type Parser t a = [t] -> (a, [t])
-- We take a stream of tokens 't' and return a result
-- of type 'a' and a remaining stream of tokens
-- without those tokens consumed to produce the result.
--
-- We will want Applicative, Alternative, and Monad
-- instances and we will want failure so that we can
-- have different branches for the parser to choose
-- from.
-- The above type is the State monad and we can use
-- the State transformer on the Maybe type to provide
-- failure and give us an 'Alternative' instance.
-- type Parser t a = StateT [t] Maybe a
-- We automatically get backtracking with this type.
-- Maybe is OK for us here as we don't care about
-- error messages and/or location information.
--
-- This type isn't going to be as performant as those
-- in the dedicated packages on hackage but we don't
-- care about that for this test and can easily swap
-- to a proper parser package.
--
-- We can build our word parser by constructing
-- parser-combinators which are combined by using
-- The various standard typeclasses we inherited from
-- the state transformer type.
-- -------------------------------------------------

-- Our parser type specialised to a list of 'Token's
type Parser a = StateT [Token] Maybe a

-- Consume and return the text from a single 'Word' 'Token'
word :: Parser Text
word = token $ elimToken (Just . Text.pack) (const Nothing) Nothing

-- Consume and return a 'Punctuation' 'Char'
takePunctuation :: Parser Char
takePunctuation = token $ elimToken (const Nothing) Just Nothing

-- Consume and return a specific 'Punctuation' 'Char'.
punctuation :: Char -> Parser Char
punctuation match = do
  p <- takePunctuation
  if p == match
    then return p
    else lift Nothing

-- Match the end of the input stream.
eof :: Parser ()
eof = StateT $ \case
  [] -> return ((), [])
  _ -> mempty

-- Consume a word or words separated only by a single hyphen
-- and combine into a single 'Text' word.
hyphenatedWord :: Parser Text
hyphenatedWord =  join' <$> word <*> many y
  where
    y = punctuation '-' *> word
    join' a b = Text.intercalate "-" (a : b)

-- Match a word and if followed by "'s" then ignore
-- those tokens.
matchPossessive :: Parser Text
matchPossessive = hyphenatedWord<* (appos <|> pure ())
  where
    appos = do
      _ <- punctuation '\''
      t <- word
      guard $ t == "s"

-- Consume a whole sentence returning only the whole
-- words and ignoring all the punctuation and spaces
sentence :: Parser [Text]
sentence = seps *> many (matchPossessive <* seps) <* eof
  where
    seps = many (space <|> anyPunctuation)
    anyPunctuation = takePunctuation $> ()

-- Consume and return the next 'Token' in the stream.
takeToken :: Parser Token
takeToken = StateT $ \case
  [] -> Nothing
  (x:r) -> Just (x, r)

-- Match the next 'Token' in the stream
-- mapping to a result.
token :: (Token -> Maybe a) -> Parser a
token f = do
  t <- takeToken
  lift $ f t

-- Consume a 'Space' 'Token'
space :: Parser ()
space = token $ elimToken n n (Just ())
  where
    n = const Nothing
