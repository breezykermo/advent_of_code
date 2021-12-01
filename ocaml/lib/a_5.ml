open Core
exception BadInput of string

let unpack x = match x with
| Some x -> x
| None -> raise (BadInput "")

let list_with_one_item ls = (unpack (List.hd ls))::[]
let list_of_string s = List.init (String.length s) ~f:(String.get s)

let rec srch (c1: char) (c2: char) (lo: int) (hi: int) (s: char list) =
    match s with
    | [] -> lo
    | c::newst ->
            match c with
            | c when Char.equal c c1 -> srch c1 c2 lo (lo + (hi-lo)/2) newst
            | c when Char.equal c c2 -> srch c1 c2 (lo + (hi-lo)/2) hi newst
            | _ -> raise (BadInput "Bad char")

let rowsearch = srch 'F' 'B' 0 128
let colsearch = srch 'L' 'R' 0 8

(* parsing logic *)
let lines = In_channel.read_lines "./data/a/5.txt"

let fst_parts = List.map lines ~f:(fun s -> String.sub s ~pos:0 ~len:7)
let snd_parts = List.map lines ~f:(fun s -> String.sub s ~pos:7 ~len:3)

let rows = List.map fst_parts ~f:(fun s -> rowsearch (list_of_string s))
let cols = List.map snd_parts ~f:(fun s -> colsearch (list_of_string s))

let seat_id row col = 8*row + col
let seat_ids = match (List.zip rows cols) with
| Ok x -> x
| Unequal_lengths -> raise (BadInput "xxxx")
let seat_ids = List.map seat_ids ~f:(fun (x,y) -> seat_id x y)

let part1 = string_of_int (List.fold seat_ids ~init:0 ~f:(fun acc vl -> max acc vl))

let find_my_seat acc vl =
    let (idx, lstlst, lst) = acc in
    if idx < 0 then acc else
        if (lstlst+1 = lst && lst+1 = vl)
        then (-lstlst, lst, vl)
        else (idx+1, lst, vl)


(* NO IDEA why this sort isn't working *)
let sorted_seat_ids = List.stable_sort seat_ids ~compare:Int.compare
(* let _ = List.map sorted_seat_ids ~f:(fun x -> print_endline (string_of_int x)) *)
(* index, lastlast, last *)
let part2 =
    let (x, y, z)= List.fold_left ~init:(0, 0, 0) ~f:find_my_seat sorted_seat_ids in
    (string_of_int x) ^ "-" ^ (string_of_int y) ^ "-" ^ (string_of_int z)

let solution = "Part 1: " ^ part1 ^"\nPart 2: " ^ part2
