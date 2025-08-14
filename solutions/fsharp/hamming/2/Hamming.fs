module Hamming

let distance (strand1: string) (strand2: string): int option =
    if (Seq.length strand1) <> (Seq.length strand2) then
        None
    else
        let dnaPairs = Seq.zip strand1 strand2
        let differingPairs = Seq.filter (fun (a, b) -> a <> b) dnaPairs
        Some(Seq.length differingPairs)