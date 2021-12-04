#[derive(Debug)]
struct SubReadings {
    depth: i32,
    position: i32,
    aim: i32,
}

fn part1(input: &str) {}

fn part2(input: &str) {
    let rgs = input.lines().fold(
        SubReadings {
            depth: 0,
            position: 0,
            aim: 0,
        },
        |cur_rgs, lne| {
            let parts: Vec<&str> = lne.split(' ').collect();
            let amt: i32 = parts[1].parse().unwrap();
            match parts[0] {
                "forward" => SubReadings {
                    depth: cur_rgs.depth + (cur_rgs.aim * amt),
                    position: cur_rgs.position + amt,
                    aim: cur_rgs.aim,
                },
                "up" => SubReadings {
                    depth: cur_rgs.depth,
                    position: cur_rgs.position,
                    aim: cur_rgs.aim - amt,
                },
                "down" => SubReadings {
                    depth: cur_rgs.depth,
                    position: cur_rgs.position,
                    aim: cur_rgs.aim + amt,
                },
                _ => cur_rgs,
            }
        },
    );
    println!("Part 2: {:?}", rgs.depth * rgs.position);
}

pub fn solve(input: String) {
    part1(&input);
    part2(&input);
}
