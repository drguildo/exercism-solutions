module Isogram

let isIsogram str =
    str
    |> Seq.filter System.Char.IsLetter
    |> Seq.map System.Char.ToLower
    |> Seq.countBy id
    |> Seq.forall (fun pair -> (snd pair) = 1)
