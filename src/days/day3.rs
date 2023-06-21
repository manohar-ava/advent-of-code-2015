use std::collections::HashSet;
use std::fs;

pub fn solution() {
    let rem = fs::read_to_string("3.txt").expect("Should have been able to read the file");
    let file_contents: Vec<&str> = rem.split("").collect();
    struct PresentDelivery {
        x: i32,
        y: i32,
        cord_set: HashSet<(i32, i32)>,
    }
    impl PresentDelivery {
        fn icrement_x(&mut self) {
            self.x = self.x + 1;
            self.set_cord()
        }
        fn icrement_y(&mut self) {
            self.y = self.y + 1;
            self.set_cord()
        }
        fn decrement_x(&mut self) {
            self.x = self.x - 1;
            self.set_cord()
        }
        fn decrement_y(&mut self) {
            self.y = self.y - 1;
            self.set_cord()
        }
        fn set_cord(&mut self) {
            self.cord_set.insert((self.x, self.y));
        }
    }
    fn check_and_set(pd: &mut PresentDelivery, value: &str) {
        match value {
            "<" => pd.decrement_x(),
            ">" => pd.icrement_x(),
            "^" => pd.icrement_y(),
            "v" => pd.decrement_y(),
            _ => print!(""),
        }
    }
    let mut first_santa = PresentDelivery {
        x: 0,
        y: 0,
        cord_set: HashSet::new(),
    };
    let mut santa = PresentDelivery {
        x: 0,
        y: 0,
        cord_set: HashSet::new(),
    };
    let mut robo_santa = PresentDelivery {
        x: 0,
        y: 0,
        cord_set: HashSet::new(),
    };
    santa.set_cord();
    robo_santa.set_cord();
    first_santa.set_cord();
    let mut is_santa = true;
    for c in file_contents {
        check_and_set(&mut first_santa, c);
        if is_santa {
            check_and_set(&mut santa, c)
        } else {
            check_and_set(&mut robo_santa, c)
        }
        is_santa = !is_santa
    }
    let new_set: HashSet<(i32, i32)> = santa
        .cord_set
        .union(&robo_santa.cord_set)
        .copied()
        .collect();
    println!("DAY - 3");
    println!(
        "first answer - {:?}  \nsecond answer - {:?}",
        first_santa.cord_set.len(),
        new_set.len()
    );
}
