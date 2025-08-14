module Pangram

let alphabet = Seq.toList "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

let isPangram (input: string): bool =
    let inputAlphabet =
        seq input
        |> Seq.filter System.Char.IsLetter
        |> Seq.map System.Char.ToUpper
        |> Seq.sort
        |> Seq.distinct
        |> Seq.toList
    inputAlphabet = alphabet
