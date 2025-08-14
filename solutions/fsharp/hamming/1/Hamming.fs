module Hamming

let distance (strand1: string) (strand2: string): int option =
    if strand1.Length <> strand2.Length then
        None
    else
        let dnaPairs = Seq.zip strand1 strand2
        let differingPairs = Seq.filter (fun (a, b) -> a <> b) dnaPairs
        Some(Seq.length differingPairs)