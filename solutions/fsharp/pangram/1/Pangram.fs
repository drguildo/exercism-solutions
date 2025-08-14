module Pangram

let alphabet = Seq.toList "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

let isPangram (input: string): bool =
    let inputChars =
        seq input
        |> Seq.filter System.Char.IsLetter
        |> Seq.map System.Char.ToUpper
        |> Seq.sort
        |> Seq.distinct
        |> Seq.toList
    inputChars = alphabet
