module Clock

type Clock = int

let hoursInDay = 24
let minutesInHour = 60
let minutesInDay = hoursInDay * minutesInHour

let create hours minutes =
    let timeInMinutes = (hours * minutesInHour) + minutes
    if timeInMinutes < 0
    then minutesInDay - (System.Math.Abs(timeInMinutes) % minutesInDay)
    else timeInMinutes % minutesInDay

let add minutes clock = create 0 (clock + minutes)

let subtract minutes clock = create 0 (clock - minutes)

let display clock = sprintf "%02d:%02d" (clock / minutesInHour) (clock % minutesInHour)
