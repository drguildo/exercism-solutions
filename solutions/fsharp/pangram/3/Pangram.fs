module Pangram

let alphabet = "abcdefghijklmnopqrstuvwxyz"

let isPangram (input: string): bool =
    let inputCharacters =
        input
        |> String.filter System.Char.IsLetter
        |> String.map System.Char.ToLower
    String.forall inputCharacters.Contains alphabet
