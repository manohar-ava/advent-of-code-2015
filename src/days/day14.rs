use std::fs;

#[derive(Debug)]
struct Horse {
    wait_time: i32,
    speed: i32,
    max_time: i32,
    total_distance: i32,
    update_counter: i32,
    wait_counter: i32,
    total_time: i32,
    score: i32,
}

impl Horse {
    fn update_every_sec(&mut self) {
        self.total_time += 1;

        if self.wait_counter == 0 {
            if self.update_counter <= self.max_time {
                if self.update_counter == self.max_time {
                    self.wait_counter = self.wait_time;
                } else {
                    self.update_counter += 1;
                    self.total_distance += self.speed;
                }
            }
        }

        if self.wait_counter > 0 {
            self.wait_counter -= 1;
            if self.wait_counter == 1 {
                self.update_counter = 0
            }
        }
    }
    fn update_score(&mut self) {
        self.score += 1
    }
}

pub fn solution() {
    let file_contents =
        fs::read_to_string("14.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = file_contents.lines().collect();
    let mut horses: Vec<Horse> = vec![];

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        let speed: i32 = words[3].parse().unwrap();
        let max_time: i32 = words[6].parse().unwrap();
        let wait_time: i32 = words[13].parse().unwrap();
        horses.push(Horse {
            wait_time: wait_time,
            speed: speed,
            max_time: max_time,
            total_distance: 0,
            update_counter: 0,
            wait_counter: 0,
            total_time: 0,
            score: 0,
        })
    }

    for _ in 1..=2503 {
        horses
            .iter_mut()
            .for_each(|x: &mut Horse| x.update_every_sec());
        let highest = horses
            .iter()
            .fold(0i32, |max_dist, val| val.total_distance.max(max_dist));
        horses.iter_mut().for_each(|x: &mut Horse| {
            if x.total_distance == highest {
                x.update_score()
            }
        });
        
    }
    let (max_distance, max_score) = horses
        .iter()
        .fold((0i32, 0i32), |(max_dist, max_score), val| {
            (val.total_distance.max(max_dist), val.score.max(max_score))
        });
        println!("first answer {:?} \n second answer {:?}", max_distance, max_score)

}
