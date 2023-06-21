use std::collections::HashMap;
use std::fs;

pub fn solution() {
    let mut wiries: HashMap<&str, u16> = HashMap::new();
    let mut wiries_2: HashMap<&str, u16> = HashMap::new();
    let file_contents =
        fs::read_to_string("7.txt").expect("Should have been able to read the file");
    let mut lines: Vec<&str> = file_contents.lines().collect();

    fn rec<'a>(lines: &Vec<&'a str>, wiries: &mut HashMap<&'a str, u16>) {
        if lines.len() == 0 {
            return;
        }
        let mut temp: Vec<&str> = vec![];
        for line in lines {
            let new_line: Vec<&str> = line.split(" ").collect();
            let len = new_line.len();
            if len == 3 {
                if new_line[0].chars().all(char::is_numeric) {
                    let val = new_line[0].to_string().parse::<u16>().unwrap();
                    wiries.insert(new_line[2], val);
                } else {
                    match wiries.get(&new_line[0]) {
                        Some(number) => {
                            wiries.insert(new_line[2], *number);
                        }
                        None => {
                            temp.push(line);
                        }
                    }
                }
            } else if len == 4 {
                if new_line[1].chars().all(char::is_numeric) {
                    let val = new_line[1].to_string().parse::<u16>().unwrap();
                    wiries.insert(new_line[3], !val);
                } else {
                    match wiries.get(&new_line[1]) {
                        Some(number) => {
                            wiries.insert(new_line[3], !*number);
                        }
                        None => {
                            temp.push(line);
                        }
                    }
                }
            } else if len == 5 {
                let lsh;
                let rsh;
                if new_line[0].chars().all(char::is_numeric) {
                    lsh = new_line[0].to_string().parse::<u16>().unwrap()
                } else {
                    match wiries.get(&new_line[0]) {
                        Some(number) => {
                            lsh = *number;
                        }
                        None => {
                            temp.push(line);
                            continue;
                        }
                    }
                }
                if new_line[2].chars().all(char::is_numeric) {
                    rsh = new_line[2].to_string().parse::<u16>().unwrap()
                } else {
                    match wiries.get(&new_line[2]) {
                        Some(number) => {
                            rsh = *number;
                        }
                        None => {
                            temp.push(line);
                            continue;
                        }
                    }
                }

                match new_line[1] {
                    "AND" => {
                        wiries.insert(new_line[4], lsh & rsh);
                    }
                    "OR" => {
                        wiries.insert(new_line[4], lsh | rsh);
                    }
                    "LSHIFT" => {
                        wiries.insert(new_line[4], lsh << rsh);
                    }
                    "RSHIFT" => {
                        wiries.insert(new_line[4], lsh >> rsh);
                    }
                    _ => {}
                }
            }
        }

        if temp.len() > 0 {
            rec(&temp, wiries)
        }
    }

    rec(&lines, &mut wiries);

    let  ans1 = match wiries.get(&"a") {
        Some(number) => *number,
        None => panic!("no a"),
    };
    let gg = ans1.to_string();
    wiries_2.insert("b", ans1);
    let mut temp_lines: Vec<String> = vec![];
    lines.iter_mut().for_each(|line| {
        let arr = line
            .split(" ")
            .map(|x| {
                let mut rem = x;
                if x == "b" {
                    rem = gg.as_str()
                }
                rem
            })
            .collect::<Vec<_>>()
            .join(" ");
        temp_lines.push(arr);
    });

    let line_2: Vec<&str> = temp_lines.iter().map(|s| &**s).collect();
    rec(&line_2, &mut wiries_2);
    let ans2 = match wiries_2.get(&"a") {
        Some(number) => *number,
        None => panic!("no a part 2"),
    };
    println!("DAY - 7");
    println!("first answer - {:?}\nsecond answer - {:?}", ans1, ans2)
}
