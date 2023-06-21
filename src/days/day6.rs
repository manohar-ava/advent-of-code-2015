use std::fs;

pub fn solution() {
    let file_contents =
        fs::read_to_string("6.txt").expect("Should have been able to read the file");
    let mut f_lights: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let mut lights: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    let lines: Vec<&str> = file_contents.lines().collect();

    fn control_lights(
        l: &mut Vec<Vec<i32>>,
        f: &mut Vec<Vec<i32>>,
        op: &str,
        f_c: Vec<&str>,
        s_c: Vec<&str>,
    ) {
        let x1: usize = f_c[0].parse().unwrap();
        let y1: usize = f_c[1].parse().unwrap();
        let x2: usize = s_c[0].parse().unwrap();
        let y2: usize = s_c[1].parse().unwrap();
        for i in x1..x2 + 1 {
            for j in y1..y2 + 1 {
                match op {
                    "+" => {
                        l[i][j] = l[i][j] + 1;
                        f[i][j] = 1;
                    }
                    "-" => {
                        if l[i][j] > 0 {
                            l[i][j] = l[i][j] - 1;
                        }
                        f[i][j] = 0;
                    }
                    "!" => {
                        l[i][j] = l[i][j] + 2;
                        if f[i][j] == 1 {
                            f[i][j] = 0
                        } else {
                            f[i][j] = 1
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() == 5 {
            let first_cord: Vec<&str> = words[2].split(",").collect();
            let sec_cord: Vec<&str> = words[4].split(",").collect();

            match words[1] {
                "on" => {
                    control_lights(&mut lights, &mut f_lights, "+", first_cord, sec_cord);
                }
                "off" => {
                    control_lights(&mut lights, &mut f_lights, "-", first_cord, sec_cord);
                }
                _ => {}
            }
        } else {
            let fc: Vec<&str> = words[1].split(",").collect();
            let sc: Vec<&str> = words[3].split(",").collect();
            control_lights(&mut lights, &mut f_lights, "!", fc, sc);
        }
    }

    let mut no_of_lights = 0;
    let mut brightness = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            brightness = brightness + lights[i][j];
            no_of_lights = no_of_lights + f_lights[i][j]
        }
    }
    println!("DAY - 6");
    println!(
        "first answer - {:?}\nsecond answer - {:?}",
        no_of_lights, brightness
    )
}
