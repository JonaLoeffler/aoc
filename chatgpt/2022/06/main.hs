import qualified Data.Set as Set
import System.Environment (getArgs)

-- | Find the first start-of-message marker in the input string.
findMarker :: Int -> String -> Int
findMarker n s = go 1 [] s
  where
    go :: Int -> [Char] -> String -> Int
    go i last [] = error "no marker found"
    go i last (c : cs) =
      let last' = c : take (n - 1) last
       in if Set.size (Set.fromList last') == n
            then i
            else go (i + 1) last' cs

main :: IO ()
main = do
  -- Read the first command line argument
  [partStr] <- getArgs
  let part = read partStr

  -- Read the input string
  input <- getContents
  let s = init input

  -- Print the result
  print (findMarker part s)
