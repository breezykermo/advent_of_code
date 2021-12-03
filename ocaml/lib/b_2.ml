open Core
exception BadInput of string

let f acc vl =
    let suffixes = String.split_on_chars vl ~on:[' '] in
    let (width, depth) = acc in
    (width + (match suffixes with
    | p::x::_ -> (match p with
                | "forward" -> int_of_string x
                | _ -> 0)
    | _ -> 0),
   depth + (match suffixes with
    | p::x::_ -> let xint = int_of_string x in (match p with
                | "up" -> -1*xint
                | "down" -> xint
                | _ -> 0)
    | _ -> 0))

let lines = In_channel.read_lines "./data/b/2.txt"
let (fw, fd) = List.fold lines ~init:(0,0) ~f:f
let part1 = string_of_int (fw * fd)
let part2 = ""
let solution = "Part 1: " ^ part1 ^ "\nPart 2: " ^ part2
