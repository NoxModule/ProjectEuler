import Lib.Cli (parseFirstArgInt)

-- Solution to Problem 1: [Multiples of 3 or 5](https://projecteuler.net/problem=1)
-- Returns sum of all multiples of 3 or 5 below given threshold.
main :: IO ()
main = do
  threshold <- parseFirstArgInt 10
  putStrLn $ "Answer: " ++ show (sum $ filter isDivisibleBy3Or5 [1 .. threshold - 1])

isDivisibleBy3Or5 :: Int -> Bool
isDivisibleBy3Or5 number = number `mod` 3 == 0 || number `mod` 5 == 0