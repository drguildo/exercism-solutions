module Raindrops

let convert n =
    [ (3, "Pling")
      (5, "Plang")
      (7, "Plong") ]
    |> List.map (fun (divisor, text) ->
        if n % divisor = 0 then text else "")
    |> String.concat ""
    |> function
    | "" -> string n
    | result -> result
