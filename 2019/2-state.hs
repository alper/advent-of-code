import System.IO
import Data.Sequence
import Control.Monad.State
import qualified Data.Text as T
import Data.Maybe

convertToInteger :: String -> Int
convertToInteger s = read s :: Int

type CompState = (Int, Seq Int)
type CompValue = Int

data Instruction = Add | Mult | Stop deriving (Show)

instruction :: State CompState Instruction
instruction = state $ \(pointer, mem) ->
    (case (Data.Sequence.lookup pointer mem) of
        Just 1 -> Add
        Just 2 -> Mult
        Just 99 -> Stop
        _ -> Stop
    , (pointer, mem))

calcul :: (Int -> Int -> Int) -> State CompState ()
calcul operator = state $ \(pointer, mem) ->
    let addr1 = Data.Sequence.lookup (pointer+1) mem
        addr2 = Data.Sequence.lookup (pointer+2) mem
        op1 = join $ Data.Sequence.lookup <$> addr1 <*> pure mem
        op2 = join $ Data.Sequence.lookup <$> addr2 <*> pure mem
        destAddr = Data.Sequence.lookup (pointer+3) mem 
        val = (operator <$> op1 <*> op2)
        newMem = Data.Sequence.update <$> destAddr <*> val <*> pure mem in
    ((), (pointer+4, fromJust newMem))

computeStep :: State CompState ()
computeStep = do
    inst <- instruction

    _ <- case inst of
        Add -> calcul (+) >> computeStep
        Mult -> calcul (*) >> computeStep
        Stop -> return ()

    return ()

a = [1,0,0,0,99]
b = [2,3,0,3,99]
c = [2,4,4,5,99,0]   
d = [1,1,1,4,99,5,6,0,99]


-- Part 2
parametrizeInput :: Seq Int -> Int -> Int -> Seq Int
parametrizeInput mem noun verb = update 2 verb (update 1 noun mem)

goal = 19690720

main :: IO()
main = do
    handle <- openFile "2-input.txt" ReadMode
    contents <- hGetContents handle

    let inputData = fromList . map convertToInteger . map T.unpack $ T.splitOn (T.pack ",") (T.pack contents)
    let updatedInputData = update 2 2 (update 1 12 inputData)

    print $ snd $ snd $ runState computeStep (0, updatedInputData)

    let outputs = [runState computeStep (0, parametrizeInput updatedInputData x y) | x <- [0..99], y <- [0..99]]
    let solution = Prelude.filter (\x -> ((Data.Sequence.lookup 0) $ snd $ snd $ x) == Just goal) outputs
    print solution
    print $ "Noun is: " ++ (show $ snd $ snd $ fromJust (Data.Sequence.lookup 1 solution))