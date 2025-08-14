module RobotName

type Robot =
    { Name: string }

let generateRobotName() =
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let r = System.Random()
    let randomLetter() = string letters.[r.Next(letters.Length)]
    let randomNumber() = 111 + r.Next(888)
    sprintf "%s%s%i" (randomLetter()) (randomLetter()) (randomNumber())


let mkRobot() = { Name = generateRobotName() }

let name robot = robot.Name

let reset robot = { robot with Name = generateRobotName() }
