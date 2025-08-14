module Accumulate

let rec accumulate (func: 'a -> 'b) (input: 'a list): 'b list =
    let rec accumulateImp accumulator = function
        | [] -> accumulator
        | head::tail -> accumulateImp (func head :: accumulator) tail
    accumulateImp [] input |> List.rev
