open Core

type counter = {
    idx: int;
    memory: int list;
    inc_count: int;
    inc_slide_3_count: int;
}

let lines = In_channel.read_lines "./data/b/1.txt"

let f ctr next =
    let cur = int_of_string next in
    let b1 = match ctr.memory with
            | p::_ -> p
            | _ -> Int.max_value in (* will always produce a negative *)
    let (fst_mem, snd_mem) = match ctr.memory with
            | a::b::c::_ -> (cur+a+b, a+b+c)
            | _ -> (0, 1) in (* will always produce a negative *)
    {
        idx = ctr.idx+1;
        memory = cur::ctr.memory;
        inc_count = ctr.inc_count + (if cur > b1 then 1 else 0);
        inc_slide_3_count = ctr.inc_slide_3_count + (if fst_mem > snd_mem then 1 else 0);
    };;

let init_counter: counter = {idx=0; memory=[]; inc_count=0; inc_slide_3_count=0}
let ctr = List.fold ~init:init_counter ~f:f lines

let part1 = string_of_int ctr.inc_count
let part2 = string_of_int ctr.inc_slide_3_count
let solution = "Part 1: " ^ part1 ^"\nPart 2: " ^ part2
