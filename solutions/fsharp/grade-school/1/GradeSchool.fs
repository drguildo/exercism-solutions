module GradeSchool

type School = Map<int, string list>

let empty: School = Map.empty

let add (student: string) (grade: int) (school: School): School =
    match Map.tryFind grade school with
    | None -> Map.add grade [ student ] school
    | Some(students) -> Map.add grade (List.sort (student :: students)) school

let roster (school: School): string list = Map.fold (fun roster _ students -> roster @ students) [] school

let grade (number: int) (school: School): string list =
    match Map.tryFind number school with
    | None -> []
    | Some(students) -> students
