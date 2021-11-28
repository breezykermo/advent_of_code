fn find_two(contents: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut res = None;
    for t in contents {
        let twin = sum - t;
        match contents.binary_search(&twin) {
            Ok(_) => {
                res = Some((*t, twin));
                break;
            }
            Err(_) => (),
        };
    }
    res
}

fn find_three(contents: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    let mut res = None;
    for t in contents {
        let twin = sum - t;
        match find_two(contents, twin) {
            Some((fst, snd)) => {
                res = Some((*t, fst, snd));
                break;
            }
            None => (),
        }
    }
    res
}

fn part1(input: String) {
    let mut contents: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    contents.sort_unstable();
    let (one, two) = find_two(&contents, 2020).unwrap();
    println!("{}", one * two);
}

fn part2(input: String) {
    let mut contents: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    contents.sort_unstable();
    let (one, two, three) = find_three(&contents, 2020).unwrap();

    println!("{}", one * two * three);
}

pub fn solve(input: String) {
    // part1(input)
    part2(input)
}
