open Core

let lines = In_channel.read_lines "./lib/data/a/1.txt"

let solution = List.fold lines ~init:"" ~f:(fun acc a -> acc^"\n"^a)
