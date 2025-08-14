module Raindrops

let convert (number: int): string =
    let raindrops =
        dict
            [ 3, "Pling"
              5, "Plang"
              7, "Plong" ]

    let divisors number = Seq.filter (fun n -> number % n = 0) raindrops.Keys

    let result =
        number
        |> divisors
        |> Seq.map (fun n -> raindrops.Item n)
        |> String.concat ""

    if result = "" then number.ToString() else result
