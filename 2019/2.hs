import System.IO
import qualified Data.Text as T
import Data.Sequence
import Data.Maybe
import Data.List
import Debug.Trace

convertToInteger :: String -> Int
convertToInteger s = read s :: Int

-- Tear this apart by using iterate and stepping over it
-- Or try this; https://hackage.haskell.org/package/base-4.12.0.0/docs/Data-List.html#v:unfoldr
-- Use Sequence instead of List
-- https://github.com/mstksg/advent-of-code-2019/blob/master/reflections.md#day-2
-- underN :: Int -> Maybe (Int, Int)
-- underN 0 = Nothing
-- underN n = Just (n, n - 1)
-- >>> unfoldr underN 10
-- [10,9,8,7,6,5,4,3,2,1]

addInst :: Int -> Int -> Int -> Seq Int -> Seq Int
addInst op1 op2 dest arr = update dest (op1+op2) arr

multInst :: Int -> Int -> Int -> Seq Int -> Seq Int
multInst op1 op2 dest arr = update dest (op1*op2) arr

type Memory = (Int, Seq Int)

compute :: Memory -> Maybe Memory
compute (pointer, mem) = 
    case Data.Sequence.lookup pointer mem of
        Nothing -> Nothing
        Just 99 -> Nothing
        Just 1 -> Just (nextPointer, addInst op1 op2 destination mem)
        Just 2 -> Just (nextPointer, multInst op1 op2 destination mem)
        _ -> Nothing
        where loc1 = fromJust(Data.Sequence.lookup (pointer+1) mem)
              op1 = fromJust(Data.Sequence.lookup loc1 mem)
              loc2 = fromJust(Data.Sequence.lookup (pointer+2) mem)
              op2 = fromJust(Data.Sequence.lookup loc2 mem)
              destination = fromJust(Data.Sequence.lookup (pointer+3) mem)
              nextPointer = pointer+4


foldStep :: Memory -> Maybe (Memory, Memory)
foldStep mem = traceShow ("bla", mem) $ case (compute mem) of
    Nothing -> Nothing
    Just nextMem -> Just (mem, nextMem)

run :: Memory -> [Memory]
run mem = Data.List.unfoldr foldStep mem

main = do
    handle <- openFile "2-input.txt" ReadMode
    contents <- hGetContents handle
    let inputData = fromList . map convertToInteger . map T.unpack $ T.splitOn (T.pack ",") (T.pack contents)
    let updatedInputData = update 2 2 (update 1 12 inputData)
    let outputData = run (0, updatedInputData)
    print outputData