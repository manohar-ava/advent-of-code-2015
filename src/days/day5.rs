use std::fs;

pub fn solution() {
    let file_contents =
        fs::read_to_string("5.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut good_line = 0;
    let mut new_good_line = 0;
    fn does_not_include_pattern(line: &str) -> bool {
        !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy"))
    }
    fn does_include_double_and_three_vowels(line: &str) -> bool {
        let mut first = true;
        let mut prev: char = ' ';
        let mut count = 0;
        let mut res = false;
        for c in line.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                count = count + 1
            }
            if !first && prev == c {
                res = true
            }
            prev = c;
            if first {
                first = !first
            };
        }
        return res && count >= 3;
    }
    fn does_include_without_overlap(line: &str) -> bool {
        for i in 0..line.len() - 3 {
            let rest = &line[i + 2..];
            let pat = &line[i..i + 2];
            if rest.contains(pat) {
                return true;
            }
        }
        return false;
    }
    fn has_repeating_chars(line: &str) -> bool {
        for i in 0..line.len() - 2 {
            let rem = line.chars().nth(i).unwrap();
            let sub = line.chars().nth(i + 2).unwrap();
            if rem == sub {
                return true;
            }
        }
        return false;
    }
    for line in lines {
        if does_not_include_pattern(line) && does_include_double_and_three_vowels(line) {
            good_line = good_line + 1
        }
        if does_include_without_overlap(line) && has_repeating_chars(line) {
            new_good_line = new_good_line + 1;
        }
    }
    println!("DAY - 5");
    println!(
        "first answer - {:?}\nsecond answer - {:?}",
        good_line, new_good_line
    );
}
