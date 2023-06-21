use std::fs;
use std::cmp;

pub fn solution() {
    let file_contents =
        fs::read_to_string("2.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut total_area: i32 = 0;
    let mut total_ribbon_area: i32 = 0;
    for line in lines {
        let items: Vec<i32> = line
            .trim()
            .split("x")
            .map(|x: &str| x.parse::<i32>().unwrap())
            .collect();
        let len_x_width: i32 = items[0] * items[1];
        let width_x_height: i32 = items[1] * items[2];
        let height_x_len: i32 = items[2] * items[0];
        let min = cmp::min(len_x_width, cmp::min(width_x_height, height_x_len));
        let rest = 2 * cmp::min(
            items[0] + items[1],
            cmp::min(items[1] + items[2], items[2] + items[0]),
        );
        total_area =
            total_area + (2 * len_x_width) + (2 * width_x_height) + (2 * height_x_len) + min;
        total_ribbon_area = total_ribbon_area + (rest + (items[0] * items[1] * items[2]));
    }
    println!("DAY - 2");
    println!(
        "first answer - {:?}  \nsecond answer - {:?}",
        total_area, total_ribbon_area
    );
}
