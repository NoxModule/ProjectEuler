main :: IO ()
main = do
  let answer = sum $ filter (\x -> x `mod` 3 == 0 || x `mod` 5 == 0) [1 .. 999]
  putStrLn ("Answer: " ++ show answer)