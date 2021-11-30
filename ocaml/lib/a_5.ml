open Core

let lines = In_channel.read_lines "./data/a/5.txt"

let solution = List.fold lines ~init:"" ~f:(fun acc a -> acc^"\n"^a)
