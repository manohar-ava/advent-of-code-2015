use std::cmp;
use std::fs;

pub fn solution() {
    let file_contents =
        fs::read_to_string("15.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut durability: Vec<i32> = vec![];
    let mut capacity: Vec<i32> = vec![];
    let mut flavour: Vec<i32> = vec![];
    let mut texture: Vec<i32> = vec![];
    let mut calories: Vec<i32> = vec![];
    for line in lines {
        let new_line = line.replace(",", "").clone();
        let words: Vec<&str> = new_line.split_whitespace().collect();
        capacity.push(words[2].parse::<i32>().unwrap());
        durability.push(words[4].parse::<i32>().unwrap());
        flavour.push(words[6].parse::<i32>().unwrap());
        texture.push(words[8].parse::<i32>().unwrap());
        calories.push(words[10].parse::<i32>().unwrap());
    }
    let mut sec_ans: i32 = 0;
    let mut first_ans: i32 = 0;
    let max = 100;
    for i in 0..=max {
        for j in 0..=max - i {
            for k in 0..=max - i - j {
                let c = get_val(&capacity, &max, &i, &j, &k);
                let d = get_val(&durability, &max, &i, &j, &k);
                let f = get_val(&flavour, &max, &i, &j, &k);
                let t = get_val(&texture, &max, &i, &j, &k);
                let cal = get_val(&calories, &max, &i, &j, &k);
                first_ans = cmp::max(first_ans, c * d * f * t);
                if cal == 500 {
                    sec_ans = cmp::max(sec_ans, c * d * f * t)
                };
            }
        }
    }

    println!("DAY - 15 ");
    println!(
        "first answer: {:?} \nsecond answer: {:?}",
        first_ans, sec_ans
    )
}

fn get_val(arr: &Vec<i32>, max: &i32, i: &i32, j: &i32, k: &i32) -> i32 {
    let val = (i * arr[0]) + ((j) * arr[1]) + ((k) * arr[2]) + ((max - i - j - k) * arr[3]);
    if val < 0 {
        return 0;
    }
    val
}
