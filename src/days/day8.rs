
use std::{fs};
use regex::Regex;

pub fn solution() {
    let file_contents =
        fs::read_to_string("8.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut count: usize = 0;
    let mut mem_count:usize = 0;
    
    for line in lines {
        let copy =  &line[1..line.len()-1];
        count += line.len();
        let r = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
        let r2: Regex = Regex::new(r"\\\\").unwrap();
        let r1: Regex =  Regex::new(r#"\\""#).unwrap();
       let replaced_hex = r.replace_all(copy, "?");
       let double_removed = r1.replace_all(&replaced_hex, "\\");
       let single_removed = r2.replace_all(&double_removed, "?");
       println!("{} {} {}",line.len() , single_removed.len(),single_removed);
        mem_count += single_removed.len();
    }
    println!("{} {} {}",count,mem_count, count - mem_count);
    println!("DAY - 8");
    // println!("first answer - {:?}\nsecond answer -", )
}
