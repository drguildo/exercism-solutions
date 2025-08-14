module GradeSchool

type School = Map<int, string list>

let empty: School = Map.empty

let grade (number: int) (school: School): string list =
    school
    |> Map.tryFind number
    |> Option.defaultValue []

let add (student: string) (number: int) (school: School): School =
    let roster = student :: (grade number school)
    Map.add number (List.sort roster) school

let roster (school: School): string list =
    school
    |> Map.toList
    |> List.collect snd
