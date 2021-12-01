#[derive(Default)]
struct Counter {
    idx: i32,
    prev: i32,
    count: i32,
}

fn part1(input: String) {
    let num_increases = input.lines().fold(Counter::default(), |acc, vl| {
        let prev = vl.parse::<i32>().unwrap();
        let count = if prev > acc.prev && acc.idx != 0 {
            acc.count + 1
        } else {
            acc.count
        };
        Counter {
            idx: acc.idx + 1,
            prev,
            count,
        }
    });
    println!("Part 1: {}", num_increases.count);
}

fn part2(input: String) {
    println!("{}", input);
}

pub fn solve(input: String) {
    part1(input)
    // part2(input)
}
