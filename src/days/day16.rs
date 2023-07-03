use std::collections::HashMap;
use std::fs;

pub fn solution() {
    let file_contents =
        fs::read_to_string("16.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let clues: HashMap<&str, &str> = [
        ("children", "3"),
        ("cats", "7"),
        ("samoyeds", "2"),
        ("pomeranians", "3"),
        ("akitas", "0"),
        ("vizslas", "0"),
        ("goldfish", "5"),
        ("trees", "3"),
        ("cars", "2"),
        ("perfumes", "1"),
    ]
    .iter()
    .cloned()
    .collect();
    let mut max_aunt =  0;
    for line in lines {
        let new_line =  line.replace(":", "").replace(",", "");
        let words:Vec<&str> =  new_line.split_whitespace().collect();
        let first_clue = is_valid(&clues, words[2], words[3]);
        let sec_clue = is_valid(&clues, words[4], words[5]);
        let third_clue = is_valid(&clues, words[6], words[7]);
        println!("{:?} {:?} {:?}",words[6],words[7],third_clue);
        if first_clue && sec_clue && third_clue {
            max_aunt+=1;
        }
    }
    println!("DAY - 16 {:?}",max_aunt);
}

fn is_valid(clues:&HashMap<&str,&str>, clue:&str, clue_value:&str)->bool{
    if !clues.contains_key(clue) || clues.get(clue).unwrap().to_owned() == clue_value {
       return true
    }
     false
}
