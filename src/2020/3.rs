fn count_trees(input: &str, right: usize, down: usize) -> i32 {
    let mut tree_count = 0;
    let mut idx = 0;
    for (line_no, line) in input.lines().enumerate() {
        if line_no % down != 0 {
            continue;
        }
        if idx > line.len() - 1 {
            idx = idx - line.len();
        }
        if line.chars().nth(idx).unwrap() == '#' {
            tree_count += 1;
        }
        idx += right;
    }
    return tree_count;
}

fn part1(input: String) {
    println!("{}", count_trees(&input, 3, 1));
}

fn part2(input: String) {
    let a = count_trees(&input, 1, 1) as i64;
    let b = count_trees(&input, 3, 1) as i64;
    let c = count_trees(&input, 5, 1) as i64;
    let d = count_trees(&input, 7, 1) as i64;
    let e = count_trees(&input, 1, 2) as i64;
    println!("{}", a * b * c * d * e);
}

pub fn solve(input: String) {
    // part1(input)
    part2(input)
}
