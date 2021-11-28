fn tuple_from<'a>(base: &'a str, splitter: &str) -> (&'a str, &'a str) {
    let items: Vec<_> = base.split(splitter).collect();
    match &items[..] {
        &[a, b, ..] => (a, b),
        _ => unreachable!(),
    }
}

fn to_size(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn get_parts(row: &str) -> (usize, usize, &str, &str) {
    let (policy, password) = tuple_from(&row, &":");
    let (rng, letter) = tuple_from(&policy, &" ");
    let (low, high) = tuple_from(&rng, &"-");
    let (low, high) = (to_size(&low), to_size(&high));
    (low, high, letter, password)
}

fn policy_one(contents: &Vec<&str>) -> i32 {
    let mut outer_cnt = 0;
    for row in contents {
        let (low, high, letter, password) = get_parts(&row);
        let cs = password.split(letter).collect::<Vec<&str>>();
        let cnt = cs.len() - 1;
        if cnt >= low && cnt <= high {
            outer_cnt += 1;
        }
    }
    outer_cnt
}

fn check_char(pw: &Vec<char>, letter: &str, pos: usize) -> bool {
    pw[pos - 1] == letter.chars().next().unwrap()
}

fn policy_two(contents: &Vec<&str>) -> i32 {
    let mut outer_cnt = 0;
    for row in contents {
        let (low, high, letter, password) = get_parts(&row);
        let pw: Vec<_> = password.strip_prefix(" ").unwrap().chars().collect();
        let is_fst = check_char(&pw, letter, low);
        let is_snd = check_char(&pw, letter, high);
        if (is_fst && !is_snd) || (!is_fst && is_snd) {
            outer_cnt += 1;
        }
    }
    outer_cnt
}
fn part1(input: String) {
    let mut contents: Vec<_> = input.lines().collect();
    println!("{}", policy_one(&contents));
}

fn part2(input: String) {
    let mut contents: Vec<_> = input.lines().collect();
    println!("{}", policy_two(&contents));
}

pub fn solve(input: String) {
    // part1(input)
    part2(input)
}
