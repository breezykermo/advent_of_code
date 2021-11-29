struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<i32>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i32>,
}

impl Passport {
    fn new(s: &str) -> Self {
        let mut slf = Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        let fs = s.split(" ").filter(|x| x.len() > 0);
        for f in fs {
            let mut cmps = f.split(":");
            let id = cmps.next().unwrap();
            let vl = cmps.next().unwrap();
            match id {
                "byr" => {
                    let byr = vl.parse::<i32>().unwrap();
                    slf.byr = if byr >= 1920 && byr <= 2002 {
                        Some(byr)
                    } else {
                        None
                    }
                }
                "iyr" => {
                    let iyr = vl.parse::<i32>().unwrap();
                    slf.iyr = if iyr >= 2010 && iyr <= 2020 {
                        Some(iyr)
                    } else {
                        None
                    }
                }
                "eyr" => {
                    let eyr = vl.parse::<i32>().unwrap();
                    slf.eyr = if eyr >= 2020 && eyr <= 2030 {
                        Some(eyr)
                    } else {
                        None
                    }
                }
                "hgt" => {
                    let hgt = vl.to_string();
                    let len = hgt.len();
                    let pfx = &hgt[..(len - 2)];
                    let sfx = &hgt[(len - 2)..];
                    slf.hgt = match sfx {
                        "cm" => {
                            let h = pfx.parse::<i32>().unwrap();
                            if h >= 150 && h <= 193 {
                                Some(h)
                            } else {
                                None
                            }
                        }
                        "in" => None,
                        _ => None,
                    };
                }
                "hcl" => {
                    slf.hcl = Some(vl.to_string());
                }
                "ecl" => {
                    slf.ecl = Some(vl.to_string());
                }
                "pid" => {
                    slf.pid = Some(vl.to_string());
                }
                "cid" => {
                    slf.cid = Some(vl.parse::<i32>().unwrap());
                }
                _ => {
                    println!("Invalid field.");
                }
            }
        }
        slf
    }

    fn is_valid(self: &Self) -> bool {
        return self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None;
    }
}

fn part1(input: String) {
    let pports: Vec<String> = input.split("\n\n").map(|x| x.replace("\n", " ")).collect();
    let pports: Vec<Passport> = pports.iter().map(|x| Passport::new(x)).collect();
    let valid_count = pports.iter().filter(|pport| pport.is_valid()).count();
    println!("Valid: {}", valid_count);
}

fn part2(input: String) {
    println!("{}", input);
}

pub fn solve(input: String) {
    part1(input);
    // part2(input)
}
