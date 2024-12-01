main :: IO ()
main = do
  let isDivisibleBy3Or5 number = number `mod` 3 == 0 || number `mod` 5 == 0
  let answer = sum $ filter isDivisibleBy3Or5 [1 .. 999]
  putStrLn $ "Answer: " ++ show answer