
    use std::fs;

    pub fn solution (){
    let file_contents = fs::read_to_string("1.txt").expect("Should have been able to read the file");
    let mut floor = 0;
    let mut found = false;
    let mut count = 1;
    let mut res = 0;
    for c in file_contents.chars() {
        match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => panic!("invalid character"),
        }
        if floor == -1 && !found {
            found = true;
            res = count
        }
        count = count + 1
    }
    println!("DAY - 1");
    println!("first answer - {floor} \nsecond answer - {res}")
    }

