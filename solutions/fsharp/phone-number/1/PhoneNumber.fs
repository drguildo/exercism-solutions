module PhoneNumber

open System

let allowedPunctuation = [ '+'; ' '; '('; ')'; '-'; '.' ]
let allowedValues = [ '0' .. '9' ] @ allowedPunctuation

let isDisallowedPunctuation c = not (List.contains c allowedValues)

let validateAreaOrExchangeCode input =
    match input with
    | '0' :: _ -> Error "cannot start with zero"
    | '1' :: _ -> Error "cannot start with one"
    | _ -> Ok input

let validateLocalNumber input =
    let exchangeCode = List.take 3 input
    printfn "exchange code: %A" exchangeCode
    match validateAreaOrExchangeCode exchangeCode with
    | Error e -> Error("exchange code " + e)
    | Ok _ -> Ok input

let validateNationalNumber input =
    let areaCode = List.take 3 input
    printfn "area code: %A" areaCode
    match validateAreaOrExchangeCode areaCode with
    | Error e -> Error("area code " + e)
    | Ok _ -> validateLocalNumber (List.skip 3 input)

let validateInternationalNumber input =
    if List.head input <> '1'
    then Error "11 digits must start with 1"
    else validateNationalNumber (List.tail input)

let clean input =
    let chars = Seq.toList input
    if List.exists Char.IsLetter chars then
        Error "letters not permitted"
    elif List.exists isDisallowedPunctuation chars then
        Error "punctuations not permitted"
    else
        let digitsOnly = List.filter Char.IsDigit chars
        let numDigits = List.length digitsOnly
        if numDigits > 11 then
            Error "more than 11 digits"
        elif numDigits = 11 then 
            match validateInternationalNumber digitsOnly with
            | Error e -> Error e
            | Ok _ -> Ok(uint64 (String.Concat(List.tail digitsOnly)))
        elif numDigits = 10 then
            match validateNationalNumber digitsOnly with
            | Error e -> Error e
            | Ok _ -> Ok(uint64 (String.Concat digitsOnly))
        else
            Error "incorrect number of digits"
