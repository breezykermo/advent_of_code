#[derive(Default)]
struct Counter {
    idx: i32,
    back1: i32,
    back2: i32,
    back3: i32,
    count: i32,
    slide_count: i32,
}

fn add_one_conditionally(count: i32, condition: bool) -> i32 {
    if condition {
        count + 1
    } else {
        count
    }
}

fn get_counter(input: &str) -> Counter {
    input.lines().fold(Counter::default(), |ctr, vl| {
        let cur = vl.parse::<i32>().unwrap();
        let fst_sum = ctr.back3 + ctr.back2 + ctr.back1;
        let snd_sum = ctr.back2 + ctr.back1 + cur;
        Counter {
            idx: ctr.idx + 1,
            back1: cur,
            back2: ctr.back1,
            back3: ctr.back2,
            count: add_one_conditionally(ctr.count, cur > ctr.back1 && ctr.idx != 0),
            slide_count: add_one_conditionally(ctr.slide_count, ctr.idx >= 3 && snd_sum > fst_sum),
        }
    })
}

fn part1(input: &str) {
    println!("Part 1: {}", get_counter(input).count);
}

fn part2(input: &str) {
    println!("Part 2: {}", get_counter(input).slide_count);
}

pub fn solve(input: String) {
    part1(&input);
    part2(&input);
}
