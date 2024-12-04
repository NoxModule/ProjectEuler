module Lib.Cli where

import Data.Maybe (fromMaybe)
import System.Environment (getArgs)
import Text.Read (readMaybe)

parseFirstArgInt :: Int -> IO Int
parseFirstArgInt defaultValue = do
  args <- getArgs
  return $ case args of
    (firstArg : _) -> fromMaybe defaultValue (readMaybe firstArg)
    [] -> defaultValue